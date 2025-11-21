//! Layout Engine
//!
//! Implements the CSS box model and layout algorithms:
//! - Block layout
//! - Inline layout
//! - Flex layout (basic)
//! - Position calculations

mod block;
pub mod box_model;
mod inline;
mod tree;

pub use box_model::{BoxModel, Dimensions, EdgeSizes, Rect};
pub use tree::{LayoutId, LayoutNode, LayoutTree};
