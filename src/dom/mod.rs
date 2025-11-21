//! DOM (Document Object Model) Implementation
//!
//! Provides:
//! - Node types (Element, Text, Document, etc.)
//! - HTML parsing via html5ever
//! - DOM tree traversal and manipulation
//! - Efficient memory layout using arena allocation

mod node;
mod document;
mod parser;
mod element;
mod attributes;

pub use node::{Node, NodeId, NodeType, NodeData};
pub use document::Document;
pub use parser::parse_html;
pub use element::Element;
pub use attributes::Attributes;
