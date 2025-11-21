//! CSS Parsing and Style System
//!
//! Implements:
//! - CSS parsing with cssparser
//! - Selector matching
//! - Specificity calculation
//! - Style cascade and inheritance
//! - Computed styles

mod context;
mod parser;
mod properties;
mod selector;
mod style;
mod values;

pub use context::StyleContext;
pub use parser::parse_stylesheet;
pub use properties::PropertyId;
pub use selector::{Selector, Specificity};
pub use style::{ComputedStyle, StyleDeclaration, StyleProperty, StyleRule};
pub use values::{Color, Display, Length};
