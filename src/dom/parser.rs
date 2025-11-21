//! HTML Parser using html5ever
//!
//! Provides standards-compliant HTML parsing that handles
//! malformed HTML gracefully (like real browsers).

use crate::dom::{Document, NodeId};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData as RcNodeData, RcDom};

/// Parse HTML string into a Document
pub fn parse_html(html: &str) -> Document {
    // Parse with html5ever
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    // Convert to our Document structure
    let mut document = Document::new();
    let root = document.root();

    // Process the tree
    convert_node(&dom.document, &mut document, root);

    document
}

/// Convert html5ever node tree to our Document structure
fn convert_node(handle: &Handle, document: &mut Document, parent_id: NodeId) {
    let node = handle;

    match &node.data {
        RcNodeData::Document => {
            // Process children of document
            for child in node.children.borrow().iter() {
                convert_node(child, document, parent_id);
            }
        }
        RcNodeData::Doctype { .. } => {
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
            let content = contents.to_string();
            let comment_id = document.create_comment(&content);
            document.append_child(parent_id, comment_id);
        }
        RcNodeData::Element { name, attrs, .. } => {
            let tag_name = name.local.to_string();
            let namespace = if name.ns == html5ever::namespace_url!("http://www.w3.org/1999/xhtml")
            {
                None
            } else {
                Some(name.ns.to_string())
            };

            // Create element
            let element_id = document.create_element(&tag_name, namespace);

            // Set attributes
            let attrs_borrowed = attrs.borrow();
            for attr in attrs_borrowed.iter() {
                document.set_attribute(
                    element_id,
                    attr.name.local.to_string(),
                    attr.value.to_string(),
                );
            }

            document.append_child(parent_id, element_id);

            // Process children
            for child in node.children.borrow().iter() {
                convert_node(child, document, element_id);
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
