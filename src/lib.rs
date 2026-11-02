//! The Sent REST API client.
//!
//! ```no_run
//! use sent::models::SendMessageV3Request;
//! use sent::Client;
//!
//! # async fn run() -> sent::Result<()> {
//! let client = Client::from_env()?; // SENT_API_KEY
//! let request: SendMessageV3Request = serde_json::from_value(serde_json::json!({
//!     "to": ["+14155551234"],
//!     "channel": ["sms"],
//!     "template": { "id": "7ba7b820-9dad-11d1-80b4-00c04fd430c8", "parameters": { "name": "Ada" } }
//! }))?;
//! let sent = client.messages().send(&request).await?;
//! println!("{:?}", sent.status);
//! # Ok(())
//! # }
//! ```
//!
//! Every resource of the API is a method on [`Client`]: `client.contacts().list(&params)`, `client.webhooks().create(&request)`.
//! The types under [`models`] and the resource modules are generated from Sent's published OpenAPI document;
//! this file, [`client`], [`error`] and [`pagination`] are written by hand.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod client;
pub mod error;
/// Every schema of the API as a serde type.
pub mod models;
pub mod pagination;
/// The resources of the API, one module each, reached from [`Client`].
pub mod resources;

pub use client::{ApiResponse, Client, ClientBuilder};
pub use error::{Error, Result};
pub use pagination::Page;

/// The version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
