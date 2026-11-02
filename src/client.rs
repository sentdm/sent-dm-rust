//! The client: authentication, the base URL, retries, idempotency keys, and the envelope every response shares.

use std::sync::Arc;
use std::time::Duration;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
pub use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::{ApiFailure, Error, Result};
use crate::models::{ApiError, ApiMeta};

/// Where the client connects when nothing else is set.
pub const DEFAULT_BASE_URL: &str = "https://api.sent.dm";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_MAX_RETRIES: u32 = 2;
const MAX_BACKOFF: Duration = Duration::from_secs(10);

/// The envelope every v3 response carries: `success`, the `data` on success, the `error` otherwise, and `meta`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Whether the request succeeded.
    pub success: bool,
    /// The payload, on success.
    #[serde(default = "none", skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// The error, on failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    /// The request id, the timestamp and the API version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<ApiMeta>,
}

fn none<T>() -> Option<T> {
    None
}

/// A query with no parameters.
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct NoQuery {}

/// One path segment, percent-encoded.
#[must_use]
pub fn encode(segment: &str) -> String {
    utf8_percent_encode(segment, NON_ALPHANUMERIC).to_string()
}

struct Inner {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
    profile_id: Option<String>,
    max_retries: u32,
}

/// The Sent API client. Cheap to clone; one per process is the usual shape.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.inner.base_url)
            .field("profile_id", &self.inner.profile_id)
            .finish_non_exhaustive()
    }
}

/// How to build a [`Client`] with more than an API key.
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    profile_id: Option<String>,
    timeout: Duration,
    max_retries: u32,
}

impl ClientBuilder {
    /// Where to send requests. The default is Sent; a mock of the API for tests, or a proxy, goes here.
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }

    /// Scope every request to a child profile: the `x-profile-id` header. Only organization API keys can use it.
    #[must_use]
    pub fn profile_id(mut self, profile_id: impl Into<String>) -> Self {
        self.profile_id = Some(profile_id.into());
        self
    }

    /// How long one attempt may take, 60 seconds by default.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// How many times a request is sent again after a 429, a 5xx or a transport error; 2 by default, 0 for none.
    /// A request that is sent again carries the same `Idempotency-Key`, so Sent performs it once.
    #[must_use]
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// The client.
    ///
    /// # Errors
    /// `Error::Config` when the API key is empty or the HTTP client cannot be built.
    pub fn build(self) -> Result<Client> {
        if self.api_key.trim().is_empty() {
            return Err(Error::Config("the API key is empty".into()));
        }
        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .user_agent(format!("sent-rust/{}", crate::VERSION))
            .build()
            .map_err(|error| {
                Error::Config(format!("the HTTP client could not be built: {error}"))
            })?;
        Ok(Client {
            inner: Arc::new(Inner {
                http,
                base_url: self.base_url,
                api_key: self.api_key,
                profile_id: self.profile_id,
                max_retries: self.max_retries,
            }),
        })
    }
}

impl Client {
    /// A client for Sent with an API key: `sk_live_...` for production, `sk_test_...` for the sandbox.
    ///
    /// # Errors
    /// `Error::Config` when the key is empty.
    pub fn new(api_key: impl Into<String>) -> Result<Client> {
        Client::builder(api_key).build()
    }

    /// A client with the API key in `SENT_API_KEY`, or `SENT_DM_API_KEY` where that older name is still set.
    ///
    /// # Errors
    /// `Error::Config` when neither variable is set.
    pub fn from_env() -> Result<Client> {
        let key = std::env::var("SENT_API_KEY")
            .or_else(|_| std::env::var("SENT_DM_API_KEY"))
            .map_err(|_| Error::Config("set SENT_API_KEY to the account's API key".into()))?;
        Client::new(key)
    }

    /// A builder, for a base URL, a profile, a timeout or a retry count.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            profile_id: None,
            timeout: DEFAULT_TIMEOUT,
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }

    /// Where requests go.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// Sends a request and reads the envelope's `data`.
    pub(crate) async fn send<T: DeserializeOwned, Q: Serialize + ?Sized, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        body: Option<&B>,
    ) -> Result<T> {
        let response = self.request(method, path, query, body).await?;
        let status = response.status();
        let text = response.text().await?;
        let envelope: ApiResponse<T> = serde_json::from_str(&text).map_err(|error| {
            Error::Decode(format!("HTTP {}: {error}: {}", status.as_u16(), cut(&text)))
        })?;
        match envelope.data {
            Some(data) if envelope.success => Ok(data),
            _ => Err(api_error(status.as_u16(), envelope.error, envelope.meta)),
        }
    }

    /// Sends a request whose success has no body.
    pub(crate) async fn send_empty<Q: Serialize + ?Sized, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        body: Option<&B>,
    ) -> Result<()> {
        self.request(method, path, query, body).await?;
        Ok(())
    }

    async fn request<Q: Serialize + ?Sized, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        body: Option<&B>,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.inner.base_url, path);
        let idempotency_key = (method != Method::GET).then(|| uuid::Uuid::new_v4().to_string());
        let mut attempt = 0;
        loop {
            let mut request = self
                .inner
                .http
                .request(method.clone(), &url)
                .header("x-api-key", &self.inner.api_key)
                .header("accept", "application/json")
                .query(query);
            if let Some(profile) = &self.inner.profile_id {
                request = request.header("x-profile-id", profile);
            }
            if let Some(key) = &idempotency_key {
                request = request.header("Idempotency-Key", key);
            }
            if let Some(body) = body {
                request = request.json(body);
            }
            let outcome = request.send().await;
            let retry_after = match &outcome {
                Ok(response) if retryable(response.status().as_u16()) => {
                    Some(retry_after(response))
                }
                Err(error) if error.is_connect() || error.is_timeout() || error.is_request() => {
                    Some(None)
                }
                _ => None,
            };
            match (outcome, retry_after) {
                (Ok(response), None) => {
                    let status = response.status();
                    if status.is_success() {
                        return Ok(response);
                    }
                    let text = response.text().await.unwrap_or_default();
                    return Err(
                        match serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                            Ok(envelope) if envelope.error.is_some() => {
                                api_error(status.as_u16(), envelope.error, envelope.meta)
                            }
                            _ => Error::Http {
                                status: status.as_u16(),
                                body: cut(&text),
                            },
                        },
                    );
                }
                (outcome, Some(wait)) if attempt < self.inner.max_retries => {
                    attempt += 1;
                    drop(outcome);
                    let backoff = wait
                        .unwrap_or_else(|| Duration::from_millis(500 * 2u64.pow(attempt - 1)))
                        .min(MAX_BACKOFF);
                    tokio::time::sleep(backoff).await;
                }
                (Ok(response), Some(_)) => {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    return Err(
                        match serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                            Ok(envelope) if envelope.error.is_some() => {
                                api_error(status.as_u16(), envelope.error, envelope.meta)
                            }
                            _ => Error::Http {
                                status: status.as_u16(),
                                body: cut(&text),
                            },
                        },
                    );
                }
                (Err(error), _) => return Err(Error::Transport(error)),
            }
        }
    }
}

fn retryable(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

fn retry_after(response: &reqwest::Response) -> Option<Duration> {
    response
        .headers()
        .get("retry-after")?
        .to_str()
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()
        .map(Duration::from_secs)
}

fn api_error(status: u16, error: Option<ApiError>, meta: Option<ApiMeta>) -> Error {
    let error = error.unwrap_or_default();
    Error::Api(Box::new(ApiFailure {
        status,
        code: error.code.unwrap_or_default(),
        message: error.message.unwrap_or_default(),
        details: error.details,
        doc_url: error.doc_url,
        request_id: meta.and_then(|meta| meta.request_id),
    }))
}

fn cut(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.chars().count() > 500 {
        format!("{}…", trimmed.chars().take(500).collect::<String>())
    } else {
        trimmed.to_string()
    }
}
