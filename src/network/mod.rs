//! Networking Layer
//!
//! Provides HTTP/HTTPS networking capabilities:
//! - Async HTTP client
//! - Cookie management
//! - Cache management
//! - Connection pooling

mod client;
mod cache;
mod error;

pub use client::{NetworkClient, Response};
pub use cache::Cache;
pub use error::NetworkError;
