//! Browser Engine - Core orchestration module
//!
//! Coordinates all browser components following the multi-process architecture.

mod engine;
mod tab;
mod error;

pub use engine::Browser;
pub use tab::Tab;
pub use error::{BrowserError, Result};
