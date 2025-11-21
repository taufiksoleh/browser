//! Layout Engine
//!
//! Implements the CSS box model and layout algorithms:
//! - Block layout
//! - Inline layout
//! - Flex layout (basic)
//! - Position calculations

mod tree;
mod box_model;
mod block;
mod inline;

pub use tree::{LayoutTree, LayoutNode, LayoutId};
pub use box_model::{BoxModel, Dimensions, Rect, EdgeSizes};
