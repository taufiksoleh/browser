//! Browser Engine - Core orchestration module
//!
//! Coordinates all browser components following the multi-process architecture.

mod engine;
mod error;
mod tab;

pub use engine::Browser;
pub use error::{BrowserError, Result};
pub use tab::Tab;
