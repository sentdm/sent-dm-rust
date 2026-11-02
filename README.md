# Sent for Rust

The Sent REST API from Rust: SMS, WhatsApp, RCS and voice through one API, with routing and compliance
handled by [Sent](https://sent.dm). `82` operations of API `v3`, every one of them a
typed method.

Status: Preview. Generated and rendered by `sent-dm-sdk-pipeline` from Sent's published OpenAPI document;
every release passes two checks: the spec's own examples decode into the types, and every operation runs
against a mock of the spec.

Read the [docs](https://docs.sent.dm) to get started.

## Install

```sh
cargo add sent tokio --features tokio/full
```

Rust 1.85 or later. The client is `reqwest` with rustls; the types are `serde`.

## Quickstart

```rust
use sent::models::SendMessageV3Request;
use sent::Client;

#[tokio::main]
async fn main() -> sent::Result<()> {
    let client = Client::from_env()?; // SENT_API_KEY
    let request: SendMessageV3Request = serde_json::from_value(serde_json::json!({
        "to": ["+14155551234"],
        "channel": ["sms"],
        "template": { "id": "7ba7b820-9dad-11d1-80b4-00c04fd430c8", "parameters": { "name": "Ada" } }
    }))?;
    let sent = client.messages().send(&request).await?;
    println!("{:?}", sent.status);
    Ok(())
}
```

Every resource is a method on the client, named as in the other Sent SDKs: `client.contacts().list(&params)`,
`client.webhooks().rotate_secret(id, &request)`, `client.voice().calls().retrieve(id)`. A list method returns a
`Page`, and its `list_all` twin walks every page.

| On the client                       | What it does                                                                                     |
| ----------------------------------- | ------------------------------------------------------------------------------------------------ |
| `Client::new(key)`                  | A client for Sent                                                                                |
| `Client::from_env()`                | The key from `SENT_API_KEY`, or `SENT_DM_API_KEY` where that older name is still set            |
| `Client::builder(key)`              | A base URL, a profile id for `x-profile-id`, a timeout, a retry count                           |

A refused request is `Error::Api` with Sent's code, message, field details and request id; a request that never
got an answer is `Error::Transport`. Every non-GET request carries an `Idempotency-Key`, kept across the retries
the client makes on 429 and 5xx, so Sent performs it once.

## Develop

```sh
cargo fmt --check && cargo clippy --all-targets -- -D warnings   # formatting, lints
cargo test                                                       # the spec's examples against the types
npx -y @stoplight/prism-cli@5 mock spec/openapi.json -p 4010 &   # a mock of the API, from the spec
SENT_BASE_URL=http://127.0.0.1:4010 cargo test --test contract   # every operation against it
```

Releases are cut by release-please from `main` and published to crates.io from GitHub Actions by trusted
publishing.
