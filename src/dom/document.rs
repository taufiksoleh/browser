//! Document - The root of the DOM tree
//!
//! Uses arena allocation for efficient memory layout and
//! O(1) node access by ID.

use crate::dom::{Attributes, Node, NodeData, NodeId};

/// The DOM Document
#[derive(Debug)]
pub struct Document {
    /// Arena of all nodes
    nodes: Vec<Node>,
    /// Root document node ID
    root: NodeId,
    /// Document element (<html>) ID
    document_element: Option<NodeId>,
    /// <head> element ID (cached for quick access)
    head: Option<NodeId>,
    /// <body> element ID (cached for quick access)
    body: Option<NodeId>,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        let mut doc = Self {
            nodes: Vec::with_capacity(256),
            root: NodeId(0),
            document_element: None,
            head: None,
            body: None,
        };

        // Create root document node
        doc.nodes.push(Node::new(NodeId(0), NodeData::Document));
        doc
    }

    /// Get root node ID
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Get document element (<html>)
    pub fn document_element(&self) -> Option<NodeId> {
        self.document_element
    }

    /// Get <head> element
    pub fn head(&self) -> Option<NodeId> {
        self.head
    }

    /// Get <body> element
    pub fn body(&self) -> Option<NodeId> {
        self.body
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id.index())
    }

    /// Get a node mutably by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id.index())
    }

    /// Create a new element node
    pub fn create_element(&mut self, tag_name: &str, namespace: Option<String>) -> NodeId {
        let id = NodeId(self.nodes.len());
        let node = Node::new(
            id,
            NodeData::Element {
                tag_name: tag_name.to_lowercase(),
                namespace,
                attributes: Attributes::new(),
            },
        );
        self.nodes.push(node);
        id
    }

    /// Create a new text node
    pub fn create_text(&mut self, content: &str) -> NodeId {
        let id = NodeId(self.nodes.len());
        let node = Node::new(
            id,
            NodeData::Text {
                content: content.to_string(),
            },
        );
        self.nodes.push(node);
        id
    }

    /// Create a new comment node
    pub fn create_comment(&mut self, content: &str) -> NodeId {
        let id = NodeId(self.nodes.len());
        let node = Node::new(
            id,
            NodeData::Comment {
                content: content.to_string(),
            },
        );
        self.nodes.push(node);
        id
    }

    /// Set an attribute on an element
    pub fn set_attribute(&mut self, node_id: NodeId, name: String, value: String) {
        if let Some(node) = self.nodes.get_mut(node_id.index()) {
            if let Some(attrs) = node.attributes_mut() {
                attrs.set(name, value);
            }
        }
    }

    /// Append a child node to a parent
    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        // Update previous sibling link
        if let Some(parent) = self.nodes.get(parent_id.index()) {
            if let Some(&last_child) = parent.children.last() {
                if let Some(last) = self.nodes.get_mut(last_child.index()) {
                    last.next_sibling = Some(child_id);
                }
                if let Some(child) = self.nodes.get_mut(child_id.index()) {
                    child.prev_sibling = Some(last_child);
                }
            }
        }

        // Set parent and add to children
        if let Some(child) = self.nodes.get_mut(child_id.index()) {
            child.parent = Some(parent_id);
        }
        if let Some(parent) = self.nodes.get_mut(parent_id.index()) {
            parent.children.push(child_id);
        }

        // Update cached references
        if let Some(node) = self.nodes.get(child_id.index()) {
            if let Some(tag) = node.tag_name() {
                match tag {
                    "html" => self.document_element = Some(child_id),
                    "head" => self.head = Some(child_id),
                    "body" => self.body = Some(child_id),
                    _ => {}
                }
            }
        }
    }

    /// Get page title
    pub fn get_title(&self) -> Option<String> {
        let head = self.head?;
        let head_node = self.get_node(head)?;

        for &child_id in &head_node.children {
            if let Some(child) = self.get_node(child_id) {
                if child.tag_name() == Some("title") {
                    // Get text content of title
                    for &text_id in &child.children {
                        if let Some(text_node) = self.get_node(text_id) {
                            if let Some(content) = text_node.text_content() {
                                return Some(content.trim().to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Find elements by tag name
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<NodeId> {
        let tag_lower = tag_name.to_lowercase();
        let mut result = Vec::new();
        self.collect_elements_by_tag(&self.root, &tag_lower, &mut result);
        result
    }

    fn collect_elements_by_tag(&self, node_id: &NodeId, tag_name: &str, result: &mut Vec<NodeId>) {
        if let Some(node) = self.get_node(*node_id) {
            if node.tag_name() == Some(tag_name) {
                result.push(*node_id);
            }
            for child_id in &node.children {
                self.collect_elements_by_tag(child_id, tag_name, result);
            }
        }
    }

    /// Find element by ID
    pub fn get_element_by_id(&self, id: &str) -> Option<NodeId> {
        self.find_element_by_id(&self.root, id)
    }

    fn find_element_by_id(&self, node_id: &NodeId, id: &str) -> Option<NodeId> {
        let node = self.get_node(*node_id)?;
        if node.get_attribute("id") == Some(id) {
            return Some(*node_id);
        }
        for child_id in &node.children {
            if let Some(found) = self.find_element_by_id(child_id, id) {
                return Some(found);
            }
        }
        None
    }

    /// Get all text content under a node
    pub fn get_text_content(&self, node_id: NodeId) -> String {
        let mut result = String::new();
        self.collect_text_content(node_id, &mut result);
        result
    }

    fn collect_text_content(&self, node_id: NodeId, result: &mut String) {
        if let Some(node) = self.get_node(node_id) {
            if let Some(text) = node.text_content() {
                result.push_str(text);
            }
            for &child_id in &node.children {
                self.collect_text_content(child_id, result);
            }
        }
    }

    /// Total number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Iterator over all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
