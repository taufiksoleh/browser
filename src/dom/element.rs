//! Element-specific functionality

use crate::dom::Attributes;

/// Element data with common operations
#[derive(Debug, Clone)]
pub struct Element {
    /// Element tag name (lowercase)
    pub tag_name: String,
    /// Namespace URI
    pub namespace: Option<String>,
    /// Element attributes
    pub attributes: Attributes,
}

impl Element {
    /// Create a new element
    pub fn new(tag_name: String, namespace: Option<String>) -> Self {
        Self {
            tag_name: tag_name.to_lowercase(),
            namespace,
            attributes: Attributes::new(),
        }
    }

    /// Create element with attributes
    pub fn with_attributes(
        tag_name: String,
        namespace: Option<String>,
        attributes: Attributes,
    ) -> Self {
        Self {
            tag_name: tag_name.to_lowercase(),
            namespace,
            attributes,
        }
    }

    /// Check if this is a void element (self-closing)
    pub fn is_void_element(&self) -> bool {
        matches!(
            self.tag_name.as_str(),
            "area"
                | "base"
                | "br"
                | "col"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "param"
                | "source"
                | "track"
                | "wbr"
        )
    }

    /// Check if this element should create a block formatting context
    pub fn creates_block_context(&self) -> bool {
        matches!(
            self.tag_name.as_str(),
            "html"
                | "body"
                | "article"
                | "section"
                | "nav"
                | "aside"
                | "header"
                | "footer"
                | "main"
                | "div"
                | "p"
                | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "h5"
                | "h6"
                | "ul"
                | "ol"
                | "li"
                | "table"
                | "form"
                | "fieldset"
                | "blockquote"
                | "pre"
                | "figure"
        )
    }

    /// Check if this is an inline element
    pub fn is_inline(&self) -> bool {
        matches!(
            self.tag_name.as_str(),
            "a" | "abbr"
                | "b"
                | "bdo"
                | "br"
                | "cite"
                | "code"
                | "dfn"
                | "em"
                | "i"
                | "img"
                | "kbd"
                | "label"
                | "q"
                | "samp"
                | "small"
                | "span"
                | "strong"
                | "sub"
                | "sup"
                | "time"
                | "var"
        )
    }
}

/// Helper for query selectors
pub struct ElementQuery;

impl ElementQuery {
    /// Check if element matches a simple selector
    pub fn matches_selector(tag_name: &str, attributes: &Attributes, selector: &str) -> bool {
        let selector = selector.trim();

        // ID selector: #id
        if let Some(id) = selector.strip_prefix('#') {
            return attributes.id() == Some(id);
        }

        // Class selector: .class
        if let Some(class) = selector.strip_prefix('.') {
            return attributes.has_class(class);
        }

        // Tag selector
        tag_name == selector.to_lowercase()
    }
}
