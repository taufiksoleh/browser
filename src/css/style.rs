//! Style rules and computed styles

use crate::css::{Selector, Specificity, PropertyId, Color, Length, Display};
use fnv::FnvHashMap;

/// A CSS style rule
#[derive(Debug, Clone)]
pub struct StyleRule {
    /// Selectors for this rule
    pub selectors: Vec<Selector>,
    /// Declarations (property -> value)
    pub declarations: Vec<StyleDeclaration>,
}

/// A single style declaration
#[derive(Debug, Clone)]
pub struct StyleDeclaration {
    pub property: PropertyId,
    pub value: String,
    pub important: bool,
}

impl StyleRule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<StyleDeclaration>) -> Self {
        Self {
            selectors,
            declarations,
        }
    }
}

/// Property value with source information
#[derive(Debug, Clone)]
pub struct StyleProperty {
    pub value: String,
    pub specificity: Specificity,
    pub important: bool,
}

impl StyleProperty {
    pub fn new(value: String, specificity: Specificity, important: bool) -> Self {
        Self {
            value,
            specificity,
            important,
        }
    }

    /// Check if this property should override another
    pub fn should_override(&self, other: &StyleProperty) -> bool {
        if self.important != other.important {
            return self.important;
        }
        self.specificity >= other.specificity
    }
}

/// Computed style for an element
#[derive(Debug, Clone)]
pub struct ComputedStyle {
    properties: FnvHashMap<PropertyId, StyleProperty>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            properties: FnvHashMap::default(),
        }
    }

    /// Set a property value
    pub fn set(&mut self, property: PropertyId, value: StyleProperty) {
        if let Some(existing) = self.properties.get(&property) {
            if !value.should_override(existing) {
                return;
            }
        }
        self.properties.insert(property, value);
    }

    /// Get a property value
    pub fn get(&self, property: PropertyId) -> Option<&str> {
        self.properties.get(&property).map(|p| p.value.as_str())
    }

    /// Get display value
    pub fn display(&self) -> Display {
        self.get(PropertyId::Display)
            .and_then(Display::parse)
            .unwrap_or(Display::Inline)
    }

    /// Get color value
    pub fn color(&self) -> Color {
        self.get(PropertyId::Color)
            .and_then(|v| Color::from_name(v).or_else(|| Color::from_hex(v)))
            .unwrap_or(Color::BLACK)
    }

    /// Get background color
    pub fn background_color(&self) -> Color {
        self.get(PropertyId::BackgroundColor)
            .and_then(|v| Color::from_name(v).or_else(|| Color::from_hex(v)))
            .unwrap_or(Color::transparent())
    }

    /// Get font size in pixels
    pub fn font_size(&self, parent_font_size: f32) -> f32 {
        self.get(PropertyId::FontSize)
            .and_then(Length::parse)
            .map(|l| l.to_px(parent_font_size, 16.0, (0.0, 0.0)))
            .unwrap_or(parent_font_size)
    }

    /// Get width
    pub fn width(&self) -> Option<Length> {
        self.get(PropertyId::Width).and_then(Length::parse)
    }

    /// Get height
    pub fn height(&self) -> Option<Length> {
        self.get(PropertyId::Height).and_then(Length::parse)
    }

    /// Get margin values (top, right, bottom, left)
    pub fn margin(&self) -> (Length, Length, Length, Length) {
        (
            self.get(PropertyId::MarginTop).and_then(Length::parse).unwrap_or(Length::Zero),
            self.get(PropertyId::MarginRight).and_then(Length::parse).unwrap_or(Length::Zero),
            self.get(PropertyId::MarginBottom).and_then(Length::parse).unwrap_or(Length::Zero),
            self.get(PropertyId::MarginLeft).and_then(Length::parse).unwrap_or(Length::Zero),
        )
    }

    /// Get padding values (top, right, bottom, left)
    pub fn padding(&self) -> (Length, Length, Length, Length) {
        (
            self.get(PropertyId::PaddingTop).and_then(Length::parse).unwrap_or(Length::Zero),
            self.get(PropertyId::PaddingRight).and_then(Length::parse).unwrap_or(Length::Zero),
            self.get(PropertyId::PaddingBottom).and_then(Length::parse).unwrap_or(Length::Zero),
            self.get(PropertyId::PaddingLeft).and_then(Length::parse).unwrap_or(Length::Zero),
        )
    }

    /// Inherit from parent style
    pub fn inherit_from(&mut self, parent: &ComputedStyle) {
        for (property, value) in &parent.properties {
            if property.is_inherited() && !self.properties.contains_key(property) {
                self.properties.insert(*property, value.clone());
            }
        }
    }

    /// Apply default values for missing properties
    pub fn apply_defaults(&mut self, is_block_element: bool) {
        // Default display based on element type
        if !self.properties.contains_key(&PropertyId::Display) {
            let display = if is_block_element { "block" } else { "inline" };
            self.set(
                PropertyId::Display,
                StyleProperty::new(display.to_string(), Specificity::default(), false),
            );
        }
    }
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self::new()
    }
}
