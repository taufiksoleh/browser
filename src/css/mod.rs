//! CSS Parsing and Style System
//!
//! Implements:
//! - CSS parsing with cssparser
//! - Selector matching
//! - Specificity calculation
//! - Style cascade and inheritance
//! - Computed styles

mod parser;
mod selector;
mod style;
mod properties;
mod context;
mod values;

pub use parser::parse_stylesheet;
pub use selector::{Selector, Specificity};
pub use style::{StyleRule, ComputedStyle, StyleProperty};
pub use context::StyleContext;
pub use properties::PropertyId;
pub use values::{CssValue, Color, Length, Display};
