//! Networking Layer
//!
//! Provides HTTP/HTTPS networking capabilities:
//! - Async HTTP client
//! - Cookie management
//! - Cache management
//! - Connection pooling

mod cache;
mod client;
mod error;

pub use cache::Cache;
pub use client::NetworkClient;
pub use error::NetworkError;
