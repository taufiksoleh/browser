//! DOM Node implementation
//!
//! Uses a flat arena-based structure for cache efficiency and
//! to avoid recursive data structures.

use crate::dom::Attributes;
use smallvec::SmallVec;

/// Node identifier - index into the document's node arena
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

/// Type of DOM node
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Document,
    DocumentType,
    Element,
    Text,
    Comment,
    ProcessingInstruction,
}

/// Node-specific data
#[derive(Debug, Clone)]
pub enum NodeData {
    Document,
    DocumentType {
        name: String,
        public_id: String,
        system_id: String,
    },
    Element {
        tag_name: String,
        namespace: Option<String>,
        attributes: Attributes,
    },
    Text {
        content: String,
    },
    Comment {
        content: String,
    },
    ProcessingInstruction {
        target: String,
        data: String,
    },
}

/// A DOM node
#[derive(Debug, Clone)]
pub struct Node {
    /// Node ID in the arena
    pub id: NodeId,
    /// Node type-specific data
    pub data: NodeData,
    /// Parent node (None for root)
    pub parent: Option<NodeId>,
    /// Child nodes (SmallVec for cache efficiency with few children)
    pub children: SmallVec<[NodeId; 8]>,
    /// Previous sibling
    pub prev_sibling: Option<NodeId>,
    /// Next sibling
    pub next_sibling: Option<NodeId>,
}

impl Node {
    /// Create a new node
    pub fn new(id: NodeId, data: NodeData) -> Self {
        Self {
            id,
            data,
            parent: None,
            children: SmallVec::new(),
            prev_sibling: None,
            next_sibling: None,
        }
    }

    /// Get node type
    pub fn node_type(&self) -> NodeType {
        match &self.data {
            NodeData::Document => NodeType::Document,
            NodeData::DocumentType { .. } => NodeType::DocumentType,
            NodeData::Element { .. } => NodeType::Element,
            NodeData::Text { .. } => NodeType::Text,
            NodeData::Comment { .. } => NodeType::Comment,
            NodeData::ProcessingInstruction { .. } => NodeType::ProcessingInstruction,
        }
    }

    /// Check if this is an element node
    pub fn is_element(&self) -> bool {
        matches!(self.data, NodeData::Element { .. })
    }

    /// Check if this is a text node
    pub fn is_text(&self) -> bool {
        matches!(self.data, NodeData::Text { .. })
    }

    /// Get element tag name
    pub fn tag_name(&self) -> Option<&str> {
        match &self.data {
            NodeData::Element { tag_name, .. } => Some(tag_name),
            _ => None,
        }
    }

    /// Get text content
    pub fn text_content(&self) -> Option<&str> {
        match &self.data {
            NodeData::Text { content } => Some(content),
            _ => None,
        }
    }

    /// Get element attributes
    pub fn attributes(&self) -> Option<&Attributes> {
        match &self.data {
            NodeData::Element { attributes, .. } => Some(attributes),
            _ => None,
        }
    }

    /// Get mutable element attributes
    pub fn attributes_mut(&mut self) -> Option<&mut Attributes> {
        match &mut self.data {
            NodeData::Element { attributes, .. } => Some(attributes),
            _ => None,
        }
    }

    /// Get attribute value
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes().and_then(|attrs| attrs.get(name))
    }
}
