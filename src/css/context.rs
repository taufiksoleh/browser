//! Style Context - Manages stylesheets and applies styles to DOM

use crate::css::{
    parse_stylesheet, ComputedStyle, PropertyId, Specificity, StyleProperty, StyleRule,
};
use crate::dom::{Document, NodeId};
use fnv::FnvHashMap;

/// Default user-agent stylesheet
const USER_AGENT_CSS: &str = r#"
html, body, div, span, p, h1, h2, h3, h4, h5, h6,
ul, ol, li, table, tr, td, th, form, fieldset, legend,
article, aside, footer, header, nav, section, main {
    display: block;
}

body {
    margin: 8px;
    font-family: sans-serif;
    font-size: 16px;
    line-height: 1.2;
}

h1 { font-size: 2em; margin: 0.67em 0; font-weight: bold; }
h2 { font-size: 1.5em; margin: 0.83em 0; font-weight: bold; }
h3 { font-size: 1.17em; margin: 1em 0; font-weight: bold; }
h4 { margin: 1.33em 0; font-weight: bold; }
h5 { font-size: 0.83em; margin: 1.67em 0; font-weight: bold; }
h6 { font-size: 0.67em; margin: 2.33em 0; font-weight: bold; }

p { margin: 1em 0; }

a { color: blue; text-decoration: underline; }

ul, ol { padding-left: 40px; margin: 1em 0; }
li { display: list-item; }

strong, b { font-weight: bold; }
em, i { font-style: italic; }

pre, code { font-family: monospace; }

img { display: inline-block; }

table { border-collapse: collapse; }
td, th { padding: 1px; }

input, textarea, select, button {
    display: inline-block;
}

[hidden] { display: none; }
"#;

/// Style context for managing and applying CSS
pub struct StyleContext {
    /// User-agent stylesheet rules
    ua_rules: Vec<StyleRule>,
    /// Author stylesheet rules (from page)
    author_rules: Vec<StyleRule>,
    /// Computed styles cache (node_id -> style)
    computed_cache: FnvHashMap<usize, ComputedStyle>,
}

impl StyleContext {
    /// Create a new style context with default UA styles
    pub fn new() -> Self {
        Self {
            ua_rules: parse_stylesheet(USER_AGENT_CSS),
            author_rules: Vec::new(),
            computed_cache: FnvHashMap::default(),
        }
    }

    /// Add a stylesheet from CSS text
    pub fn add_stylesheet(&mut self, css: &str) {
        let rules = parse_stylesheet(css);
        self.author_rules.extend(rules);
        // Clear cache when styles change
        self.computed_cache.clear();
    }

    /// Clear all author styles
    pub fn clear_author_styles(&mut self) {
        self.author_rules.clear();
        self.computed_cache.clear();
    }

    /// Apply styles to the entire document
    pub fn apply_styles(&mut self, doc: &mut Document) {
        self.computed_cache.clear();

        // Extract style elements from document
        let style_nodes = doc.get_elements_by_tag_name("style");
        for style_id in style_nodes {
            let css = doc.get_text_content(style_id);
            self.add_stylesheet(&css);
        }

        // Process link elements for external stylesheets would go here
        // (requires async network fetch)

        // Compute styles for all nodes
        let root = doc.root();
        self.compute_styles_recursive(doc, root, None);
    }

    /// Recursively compute styles for a node and its children
    fn compute_styles_recursive(
        &mut self,
        doc: &Document,
        node_id: NodeId,
        parent_style: Option<&ComputedStyle>,
    ) {
        let mut style = ComputedStyle::new();

        // Get matching rules for this node
        if let Some(node) = doc.get_node(node_id) {
            if node.is_element() {
                // Apply UA rules
                for rule in &self.ua_rules {
                    self.apply_matching_rule(doc, node_id, rule, &mut style);
                }

                // Apply author rules
                for rule in &self.author_rules {
                    self.apply_matching_rule(doc, node_id, rule, &mut style);
                }

                // Apply inline styles
                if let Some(inline) = node.get_attribute("style") {
                    self.apply_inline_styles(inline, &mut style);
                }

                // Inherit from parent
                if let Some(parent) = parent_style {
                    style.inherit_from(parent);
                }

                // Apply defaults based on element type
                let is_block = node
                    .tag_name()
                    .map(|t| is_block_element(t))
                    .unwrap_or(false);
                style.apply_defaults(is_block);
            }
        }

        // Cache the computed style
        self.computed_cache.insert(node_id.index(), style.clone());

        // Process children
        if let Some(node) = doc.get_node(node_id) {
            let children: Vec<_> = node.children.iter().copied().collect();
            for child_id in children {
                self.compute_styles_recursive(doc, child_id, Some(&style));
            }
        }
    }

    /// Apply a matching rule to a style
    fn apply_matching_rule(
        &self,
        doc: &Document,
        node_id: NodeId,
        rule: &StyleRule,
        style: &mut ComputedStyle,
    ) {
        for selector in &rule.selectors {
            if selector.matches(doc, node_id) {
                let specificity = selector.specificity();
                for decl in &rule.declarations {
                    style.set(
                        decl.property,
                        StyleProperty::new(decl.value.clone(), specificity, decl.important),
                    );
                }
            }
        }
    }

    /// Apply inline styles
    fn apply_inline_styles(&self, inline: &str, style: &mut ComputedStyle) {
        // Inline styles have highest specificity (1, 0, 0, 0)
        let inline_specificity = Specificity::new(1, 0, 0, 0);

        for decl in inline.split(';') {
            let decl = decl.trim();
            if let Some(colon_pos) = decl.find(':') {
                let property_name = decl[..colon_pos].trim();
                let value = decl[colon_pos + 1..].trim();

                let property = PropertyId::from_name(property_name);
                style.set(
                    property,
                    StyleProperty::new(value.to_string(), inline_specificity, false),
                );
            }
        }
    }

    /// Get computed style for a node
    pub fn get_computed_style(&self, node_id: NodeId) -> Option<&ComputedStyle> {
        self.computed_cache.get(&node_id.index())
    }
}

impl Default for StyleContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if an element is a block element by default
fn is_block_element(tag: &str) -> bool {
    matches!(
        tag,
        "html"
            | "body"
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
            | "tr"
            | "form"
            | "fieldset"
            | "article"
            | "aside"
            | "footer"
            | "header"
            | "nav"
            | "section"
            | "main"
            | "blockquote"
            | "pre"
            | "figure"
            | "figcaption"
            | "hr"
    )
}
