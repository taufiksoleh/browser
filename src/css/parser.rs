//! CSS Parser
//!
//! Parses CSS stylesheets into structured rules.

use crate::css::{PropertyId, Selector, StyleDeclaration, StyleRule};

/// Parse a CSS stylesheet string
pub fn parse_stylesheet(css: &str) -> Vec<StyleRule> {
    let mut rules = Vec::new();
    let css = css.trim();

    // Simple state machine parser for CSS
    let mut chars = css.chars().peekable();
    let mut current_selectors = String::new();
    let mut in_block = false;
    let mut block_content = String::new();
    let mut brace_depth = 0;

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                if !in_block {
                    in_block = true;
                    brace_depth = 1;
                } else {
                    brace_depth += 1;
                    block_content.push(c);
                }
            }
            '}' => {
                brace_depth -= 1;
                if brace_depth == 0 {
                    // End of rule block
                    if let Some(rule) = parse_rule(&current_selectors, &block_content) {
                        rules.push(rule);
                    }
                    current_selectors.clear();
                    block_content.clear();
                    in_block = false;
                } else {
                    block_content.push(c);
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                // Skip CSS comment
                chars.next(); // consume *
                while let Some(c) = chars.next() {
                    if c == '*' && chars.peek() == Some(&'/') {
                        chars.next();
                        break;
                    }
                }
            }
            _ => {
                if in_block {
                    block_content.push(c);
                } else {
                    current_selectors.push(c);
                }
            }
        }
    }

    rules
}

/// Parse a single rule from selector and declaration block
fn parse_rule(selectors_str: &str, declarations_str: &str) -> Option<StyleRule> {
    // Parse selectors (comma-separated)
    let selectors: Vec<Selector> = selectors_str
        .split(',')
        .filter_map(|s| Selector::parse(s.trim()))
        .collect();

    if selectors.is_empty() {
        return None;
    }

    // Parse declarations (semicolon-separated)
    let declarations = parse_declarations(declarations_str);

    Some(StyleRule::new(selectors, declarations))
}

/// Parse declaration block content
fn parse_declarations(content: &str) -> Vec<StyleDeclaration> {
    let mut declarations = Vec::new();

    for decl in content.split(';') {
        let decl = decl.trim();
        if decl.is_empty() {
            continue;
        }

        if let Some(colon_pos) = decl.find(':') {
            let property_name = decl[..colon_pos].trim();
            let mut value = decl[colon_pos + 1..].trim().to_string();

            // Check for !important
            let important = if value.to_lowercase().ends_with("!important") {
                value = value[..value.len() - 10].trim().to_string();
                true
            } else {
                false
            };

            let property = PropertyId::from_name(property_name);

            // Handle shorthand properties
            let expanded = expand_shorthand(property, &value);
            for (prop, val) in expanded {
                declarations.push(StyleDeclaration {
                    property: prop,
                    value: val,
                    important,
                });
            }
        }
    }

    declarations
}

/// Expand shorthand properties into individual properties
fn expand_shorthand(property: PropertyId, value: &str) -> Vec<(PropertyId, String)> {
    match property {
        PropertyId::Margin => {
            let parts: Vec<&str> = value.split_whitespace().collect();
            let (top, right, bottom, left) = match parts.len() {
                1 => (parts[0], parts[0], parts[0], parts[0]),
                2 => (parts[0], parts[1], parts[0], parts[1]),
                3 => (parts[0], parts[1], parts[2], parts[1]),
                4 => (parts[0], parts[1], parts[2], parts[3]),
                _ => return vec![(property, value.to_string())],
            };
            vec![
                (PropertyId::MarginTop, top.to_string()),
                (PropertyId::MarginRight, right.to_string()),
                (PropertyId::MarginBottom, bottom.to_string()),
                (PropertyId::MarginLeft, left.to_string()),
            ]
        }
        PropertyId::Padding => {
            let parts: Vec<&str> = value.split_whitespace().collect();
            let (top, right, bottom, left) = match parts.len() {
                1 => (parts[0], parts[0], parts[0], parts[0]),
                2 => (parts[0], parts[1], parts[0], parts[1]),
                3 => (parts[0], parts[1], parts[2], parts[1]),
                4 => (parts[0], parts[1], parts[2], parts[3]),
                _ => return vec![(property, value.to_string())],
            };
            vec![
                (PropertyId::PaddingTop, top.to_string()),
                (PropertyId::PaddingRight, right.to_string()),
                (PropertyId::PaddingBottom, bottom.to_string()),
                (PropertyId::PaddingLeft, left.to_string()),
            ]
        }
        PropertyId::Background => {
            // Simple handling - treat as background-color if it looks like a color
            if value.starts_with('#') || value.starts_with("rgb") || Color::from_name_exists(value)
            {
                vec![(PropertyId::BackgroundColor, value.to_string())]
            } else {
                vec![(property, value.to_string())]
            }
        }
        _ => vec![(property, value.to_string())],
    }
}

use crate::css::Color;

impl Color {
    /// Check if a color name exists (without allocating)
    pub fn from_name_exists(name: &str) -> bool {
        matches!(
            name.to_lowercase().as_str(),
            "black"
                | "white"
                | "red"
                | "green"
                | "blue"
                | "transparent"
                | "yellow"
                | "orange"
                | "purple"
                | "pink"
                | "gray"
                | "grey"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_stylesheet() {
        let css = r#"
            body {
                background-color: white;
                color: black;
            }
            .container {
                margin: 10px 20px;
                padding: 5px;
            }
        "#;

        let rules = parse_stylesheet(css);
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_parse_multiple_selectors() {
        let css = "h1, h2, h3 { font-weight: bold; }";
        let rules = parse_stylesheet(css);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selectors.len(), 3);
    }
}
