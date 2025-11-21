//! DOM (Document Object Model) Implementation
//!
//! Provides:
//! - Node types (Element, Text, Document, etc.)
//! - HTML parsing via html5ever
//! - DOM tree traversal and manipulation
//! - Efficient memory layout using arena allocation

mod attributes;
mod document;
mod element;
mod node;
mod parser;

pub use attributes::Attributes;
pub use document::Document;
pub use node::{Node, NodeData, NodeId, NodeType};
pub use parser::parse_html;
