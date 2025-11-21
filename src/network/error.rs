//! Network error types

use thiserror::Error;

/// Network operation errors
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Timeout")]
    Timeout,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("SSL/TLS error: {0}")]
    Ssl(String),

    #[error("DNS resolution failed: {0}")]
    DnsError(String),

    #[error("Too many redirects")]
    TooManyRedirects,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<reqwest::Error> for NetworkError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            NetworkError::Timeout
        } else if err.is_connect() {
            NetworkError::Connection(err.to_string())
        } else if err.is_redirect() {
            NetworkError::TooManyRedirects
        } else {
            NetworkError::RequestFailed(err.to_string())
        }
    }
}
