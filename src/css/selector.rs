//! CSS Selector implementation
//!
//! Supports basic selectors:
//! - Type selectors (div, p, span)
//! - Class selectors (.class)
//! - ID selectors (#id)
//! - Attribute selectors ([attr], [attr=value])
//! - Combinators (descendant, child, adjacent)
//! - Pseudo-classes (:hover, :first-child, etc.)

use crate::dom::{Document, Node, NodeId};

/// CSS Specificity (a, b, c, d) where:
/// a = inline styles (always 0 for stylesheets)
/// b = # of ID selectors
/// c = # of class, attribute, pseudo-class selectors
/// d = # of type and pseudo-element selectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Specificity(pub u32, pub u32, pub u32, pub u32);

impl Specificity {
    pub fn new(inline: u32, ids: u32, classes: u32, types: u32) -> Self {
        Self(inline, ids, classes, types)
    }

    /// Calculate numeric value for comparison
    pub fn value(&self) -> u32 {
        self.0 * 1000000 + self.1 * 10000 + self.2 * 100 + self.3
    }
}

/// Selector component
#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPart {
    /// Universal selector (*)
    Universal,
    /// Type selector (element name)
    Type(String),
    /// ID selector (#id)
    Id(String),
    /// Class selector (.class)
    Class(String),
    /// Attribute selector [attr] or [attr=value]
    Attribute {
        name: String,
        op: Option<AttributeOp>,
        value: Option<String>,
    },
    /// Pseudo-class (:hover, :first-child, etc.)
    PseudoClass(String),
    /// Pseudo-element (::before, ::after)
    PseudoElement(String),
}

/// Attribute selector operators
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeOp {
    /// [attr=value] - Exact match
    Exact,
    /// [attr~=value] - Contains word
    Contains,
    /// [attr|=value] - Starts with (hyphen-separated)
    StartsWith,
    /// [attr^=value] - Prefix
    Prefix,
    /// [attr$=value] - Suffix
    Suffix,
    /// [attr*=value] - Substring
    Substring,
}

/// Combinator between selector parts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Combinator {
    /// Descendant (space)
    Descendant,
    /// Child (>)
    Child,
    /// Adjacent sibling (+)
    Adjacent,
    /// General sibling (~)
    Sibling,
}

/// A complete selector (may have multiple parts with combinators)
#[derive(Debug, Clone)]
pub struct Selector {
    /// Parts of the selector
    pub parts: Vec<(Vec<SelectorPart>, Option<Combinator>)>,
    /// Cached specificity
    specificity: Specificity,
}

impl Selector {
    /// Parse a selector string
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }

        let mut parts = Vec::new();
        let mut current_parts = Vec::new();
        let mut chars = input.chars().peekable();
        let mut buffer = String::new();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '>' | '+' | '~' => {
                    // Flush buffer
                    if !buffer.is_empty() {
                        current_parts.push(SelectorPart::Type(buffer.to_lowercase()));
                        buffer.clear();
                    }

                    // Skip whitespace
                    while chars.peek() == Some(&' ') {
                        chars.next();
                    }

                    let combinator = match chars.peek() {
                        Some('>') => {
                            chars.next();
                            while chars.peek() == Some(&' ') {
                                chars.next();
                            }
                            Combinator::Child
                        }
                        Some('+') => {
                            chars.next();
                            while chars.peek() == Some(&' ') {
                                chars.next();
                            }
                            Combinator::Adjacent
                        }
                        Some('~') => {
                            chars.next();
                            while chars.peek() == Some(&' ') {
                                chars.next();
                            }
                            Combinator::Sibling
                        }
                        _ => Combinator::Descendant,
                    };

                    if !current_parts.is_empty() {
                        parts.push((current_parts, Some(combinator)));
                        current_parts = Vec::new();
                    }
                }
                '#' => {
                    if !buffer.is_empty() {
                        current_parts.push(SelectorPart::Type(buffer.to_lowercase()));
                        buffer.clear();
                    }
                    chars.next();
                    let id: String = chars
                        .by_ref()
                        .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                        .collect();
                    current_parts.push(SelectorPart::Id(id));
                }
                '.' => {
                    if !buffer.is_empty() {
                        current_parts.push(SelectorPart::Type(buffer.to_lowercase()));
                        buffer.clear();
                    }
                    chars.next();
                    let class: String = chars
                        .by_ref()
                        .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                        .collect();
                    current_parts.push(SelectorPart::Class(class));
                }
                '*' => {
                    chars.next();
                    current_parts.push(SelectorPart::Universal);
                }
                '[' => {
                    if !buffer.is_empty() {
                        current_parts.push(SelectorPart::Type(buffer.to_lowercase()));
                        buffer.clear();
                    }
                    chars.next();
                    // Parse attribute selector
                    let attr_str: String = chars.by_ref().take_while(|c| *c != ']').collect();
                    if let Some(part) = Self::parse_attribute(&attr_str) {
                        current_parts.push(part);
                    }
                }
                ':' => {
                    if !buffer.is_empty() {
                        current_parts.push(SelectorPart::Type(buffer.to_lowercase()));
                        buffer.clear();
                    }
                    chars.next();
                    let pseudo: String = chars
                        .by_ref()
                        .take_while(|c| c.is_alphanumeric() || *c == '-')
                        .collect();
                    current_parts.push(SelectorPart::PseudoClass(pseudo));
                }
                _ => {
                    buffer.push(chars.next().unwrap());
                }
            }
        }

        // Flush remaining buffer
        if !buffer.is_empty() {
            current_parts.push(SelectorPart::Type(buffer.to_lowercase()));
        }

        if !current_parts.is_empty() {
            parts.push((current_parts, None));
        }

        if parts.is_empty() {
            return None;
        }

        let specificity = Self::calculate_specificity(&parts);

        Some(Self { parts, specificity })
    }

    fn parse_attribute(s: &str) -> Option<SelectorPart> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        // Simple [attr] case
        if !s.contains('=') {
            return Some(SelectorPart::Attribute {
                name: s.to_string(),
                op: None,
                value: None,
            });
        }

        // [attr=value] cases
        let (name, op, value) = if let Some(pos) = s.find('=') {
            let (n, v) = s.split_at(pos);
            let v = &v[1..]; // Skip =
            let (n, op) = if let Some(stripped) = n.strip_suffix('~') {
                (stripped, AttributeOp::Contains)
            } else if let Some(stripped) = n.strip_suffix('|') {
                (stripped, AttributeOp::StartsWith)
            } else if let Some(stripped) = n.strip_suffix('^') {
                (stripped, AttributeOp::Prefix)
            } else if let Some(stripped) = n.strip_suffix('$') {
                (stripped, AttributeOp::Suffix)
            } else if let Some(stripped) = n.strip_suffix('*') {
                (stripped, AttributeOp::Substring)
            } else {
                (n, AttributeOp::Exact)
            };
            (
                n.to_string(),
                Some(op),
                Some(v.trim_matches('"').trim_matches('\'').to_string()),
            )
        } else {
            return None;
        };

        Some(SelectorPart::Attribute { name, op, value })
    }

    fn calculate_specificity(parts: &[(Vec<SelectorPart>, Option<Combinator>)]) -> Specificity {
        let mut ids = 0u32;
        let mut classes = 0u32;
        let mut types = 0u32;

        for (part_list, _) in parts {
            for part in part_list {
                match part {
                    SelectorPart::Id(_) => ids += 1,
                    SelectorPart::Class(_)
                    | SelectorPart::Attribute { .. }
                    | SelectorPart::PseudoClass(_) => classes += 1,
                    SelectorPart::Type(_) | SelectorPart::PseudoElement(_) => types += 1,
                    SelectorPart::Universal => {}
                }
            }
        }

        Specificity::new(0, ids, classes, types)
    }

    /// Get selector specificity
    pub fn specificity(&self) -> Specificity {
        self.specificity
    }

    /// Check if selector matches a node
    pub fn matches(&self, doc: &Document, node_id: NodeId) -> bool {
        self.matches_with_context(doc, node_id, self.parts.len().saturating_sub(1))
    }

    fn matches_with_context(&self, doc: &Document, node_id: NodeId, part_index: usize) -> bool {
        let node = match doc.get_node(node_id) {
            Some(n) => n,
            None => return false,
        };

        let (ref parts, ref combinator) = self.parts[part_index];

        // Check if current node matches all parts
        if !Self::node_matches_parts(node, parts) {
            return false;
        }

        // If this is the first part, we're done
        if part_index == 0 {
            return true;
        }

        // Check combinator with parent/sibling
        let prev_index = part_index - 1;
        match combinator {
            None => true,
            Some(Combinator::Child) => {
                if let Some(parent_id) = node.parent {
                    self.matches_with_context(doc, parent_id, prev_index)
                } else {
                    false
                }
            }
            Some(Combinator::Descendant) => {
                let mut current = node.parent;
                while let Some(ancestor_id) = current {
                    if self.matches_with_context(doc, ancestor_id, prev_index) {
                        return true;
                    }
                    current = doc.get_node(ancestor_id).and_then(|n| n.parent);
                }
                false
            }
            Some(Combinator::Adjacent) => {
                if let Some(prev_id) = node.prev_sibling {
                    self.matches_with_context(doc, prev_id, prev_index)
                } else {
                    false
                }
            }
            Some(Combinator::Sibling) => {
                let mut current = node.prev_sibling;
                while let Some(sib_id) = current {
                    if self.matches_with_context(doc, sib_id, prev_index) {
                        return true;
                    }
                    current = doc.get_node(sib_id).and_then(|n| n.prev_sibling);
                }
                false
            }
        }
    }

    fn node_matches_parts(node: &Node, parts: &[SelectorPart]) -> bool {
        for part in parts {
            if !Self::node_matches_part(node, part) {
                return false;
            }
        }
        true
    }

    fn node_matches_part(node: &Node, part: &SelectorPart) -> bool {
        match part {
            SelectorPart::Universal => true,
            SelectorPart::Type(tag) => node.tag_name() == Some(tag.as_str()),
            SelectorPart::Id(id) => node.get_attribute("id") == Some(id.as_str()),
            SelectorPart::Class(class) => node
                .attributes()
                .map(|a| a.has_class(class))
                .unwrap_or(false),
            SelectorPart::Attribute { name, op, value } => {
                let attr_value = node.get_attribute(name);
                match (op, value, attr_value) {
                    (None, _, Some(_)) => true,
                    (None, _, None) => false,
                    (Some(_), _, None) => false,
                    (Some(AttributeOp::Exact), Some(v), Some(a)) => a == v,
                    (Some(AttributeOp::Contains), Some(v), Some(a)) => {
                        a.split_whitespace().any(|w| w == v)
                    }
                    (Some(AttributeOp::Prefix), Some(v), Some(a)) => a.starts_with(v.as_str()),
                    (Some(AttributeOp::Suffix), Some(v), Some(a)) => a.ends_with(v.as_str()),
                    (Some(AttributeOp::Substring), Some(v), Some(a)) => a.contains(v.as_str()),
                    (Some(AttributeOp::StartsWith), Some(v), Some(a)) => {
                        a == v || a.starts_with(&format!("{}-", v))
                    }
                    _ => false,
                }
            }
            SelectorPart::PseudoClass(_) => {
                // TODO: Implement pseudo-class matching
                false
            }
            SelectorPart::PseudoElement(_) => false,
        }
    }
}
