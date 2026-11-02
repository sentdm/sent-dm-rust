//! What a request can fail with.

use std::collections::HashMap;

/// A request that did not end in the response the API documents.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Sent answered with an error: the status, and the code and message of its error envelope.
    #[error("{0}")]
    Api(Box<ApiFailure>),
    /// A status outside 2xx with a body that is not Sent's error envelope.
    #[error("HTTP {status}: {body}")]
    Http {
        /// The HTTP status.
        status: u16,
        /// The body, as text, cut to a reasonable length.
        body: String,
    },
    /// The request never got an answer: DNS, TLS, a timeout, a dropped connection.
    #[error("the request could not be completed: {0}")]
    Transport(#[from] reqwest::Error),
    /// A 2xx answer whose body is not what the API documents.
    #[error("the response could not be read: {0}")]
    Decode(String),
    /// The client is not configured well enough to send anything.
    #[error("{0}")]
    Config(String),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Decode(error.to_string())
    }
}

/// What Sent said when it refused a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiFailure {
    /// The HTTP status.
    pub status: u16,
    /// The machine-readable error code, such as `VALIDATION_004`.
    pub code: String,
    /// The message for people. It may change without notice.
    pub message: String,
    /// Field-level validation errors, when there are any.
    pub details: Option<HashMap<String, Vec<String>>>,
    /// Where the error is documented.
    pub doc_url: Option<String>,
    /// The request id Sent assigned, for support.
    pub request_id: Option<String>,
}

impl std::fmt::Display for ApiFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sent refused the request: {}: {} (HTTP {})",
            self.code, self.message, self.status
        )?;
        if let Some(id) = &self.request_id {
            write!(f, ", request {id}")?;
        }
        Ok(())
    }
}

impl Error {
    /// The failure Sent reported, when the error is one.
    #[must_use]
    pub fn api(&self) -> Option<&ApiFailure> {
        match self {
            Error::Api(failure) => Some(failure),
            _ => None,
        }
    }
}

/// The result of a request.
pub type Result<T> = std::result::Result<T, Error>;
