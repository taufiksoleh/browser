//! Browser error types

use thiserror::Error;

/// Result type for browser operations
pub type Result<T> = std::result::Result<T, BrowserError>;

/// Browser error types
#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("Network error: {0}")]
    Network(#[from] crate::network::NetworkError),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Render error: {0}")]
    Render(String),

    #[error("Layout error: {0}")]
    Layout(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Window error: {0}")]
    Window(String),

    #[error("GPU error: {0}")]
    Gpu(String),
}
