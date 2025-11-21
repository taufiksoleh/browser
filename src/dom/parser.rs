//! HTML Parser using html5ever
//!
//! Provides standards-compliant HTML parsing that handles
//! malformed HTML gracefully (like real browsers).

use crate::dom::{Document, NodeId, NodeData, Attributes};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData as RcNodeData, RcDom};
use std::collections::HashMap;

/// Parse HTML string into a Document
pub fn parse_html(html: &str) -> Document {
    // Parse with html5ever
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    // Convert to our Document structure
    let mut document = Document::new();
    let mut node_map: HashMap<usize, NodeId> = HashMap::new();

    // Process the tree
    convert_node(&dom.document, &mut document, document.root(), &mut node_map);

    document
}

/// Convert html5ever node tree to our Document structure
fn convert_node(
    handle: &Handle,
    document: &mut Document,
    parent_id: NodeId,
    node_map: &mut HashMap<usize, NodeId>,
) {
    let node = handle;

    match &node.data {
        RcNodeData::Document => {
            // Process children of document
            for child in node.children.borrow().iter() {
                convert_node(child, document, parent_id, node_map);
            }
        }
        RcNodeData::Doctype { name, .. } => {
            // Skip doctype for now, it's not critical for rendering
        }
        RcNodeData::Text { contents } => {
            let text = contents.borrow().to_string();
            // Skip whitespace-only text nodes at top level
            if !text.trim().is_empty() || parent_id.index() > 0 {
                let text_id = document.create_text(&text);
                document.append_child(parent_id, text_id);
            }
        }
        RcNodeData::Comment { contents } => {
            let comment_id = document.create_comment(&contents.borrow());
            document.append_child(parent_id, comment_id);
        }
        RcNodeData::Element { name, attrs, .. } => {
            let tag_name = name.local.to_string();
            let namespace = match name.ns {
                html5ever::ns!(html) => None,
                ref ns => Some(ns.to_string()),
            };

            // Create element
            let element_id = document.create_element(&tag_name, namespace);

            // Set attributes
            if let Some(node) = document.get_node_mut(element_id) {
                if let Some(node_attrs) = node.attributes_mut() {
                    for attr in attrs.borrow().iter() {
                        node_attrs.set(
                            attr.name.local.to_string(),
                            attr.value.to_string(),
                        );
                    }
                }
            }

            document.append_child(parent_id, element_id);

            // Process children
            for child in node.children.borrow().iter() {
                convert_node(child, document, element_id, node_map);
            }
        }
        RcNodeData::ProcessingInstruction { .. } => {
            // Skip processing instructions
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test</title></head>
            <body><p>Hello World</p></body>
            </html>
        "#;

        let doc = parse_html(html);
        assert!(doc.document_element().is_some());
        assert!(doc.head().is_some());
        assert!(doc.body().is_some());
        assert_eq!(doc.get_title(), Some("Test".to_string()));
    }

    #[test]
    fn test_parse_malformed_html() {
        // html5ever should handle this gracefully
        let html = "<p>Unclosed paragraph<div>Nested incorrectly</p></div>";
        let doc = parse_html(html);
        assert!(doc.body().is_some());
    }
}
