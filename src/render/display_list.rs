//! Display List - Intermediate representation for painting

use crate::css::Color;
use crate::layout::Rect;

/// Display command types
#[derive(Debug, Clone)]
pub enum DisplayCommand {
    /// Fill a rectangle with solid color
    SolidRect {
        rect: Rect,
        color: Color,
    },
    /// Draw text
    Text {
        text: String,
        x: f32,
        y: f32,
        color: Color,
        font_size: f32,
    },
    /// Draw border
    Border {
        rect: Rect,
        widths: (f32, f32, f32, f32), // top, right, bottom, left
        colors: (Color, Color, Color, Color),
    },
    /// Draw image
    Image {
        rect: Rect,
        image_id: u32,
    },
    /// Push clip rectangle
    PushClip(Rect),
    /// Pop clip rectangle
    PopClip,
    /// Set opacity for following commands
    PushOpacity(f32),
    /// Restore opacity
    PopOpacity,
}

/// A single display item with z-order
#[derive(Debug, Clone)]
pub struct DisplayItem {
    pub command: DisplayCommand,
    pub z_index: i32,
}

impl DisplayItem {
    pub fn new(command: DisplayCommand, z_index: i32) -> Self {
        Self { command, z_index }
    }
}

/// Display list containing all paint commands
#[derive(Debug, Default)]
pub struct DisplayList {
    items: Vec<DisplayItem>,
}

impl DisplayList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a display item
    pub fn push(&mut self, item: DisplayItem) {
        self.items.push(item);
    }

    /// Add a solid rectangle
    pub fn push_rect(&mut self, rect: Rect, color: Color, z_index: i32) {
        self.push(DisplayItem::new(
            DisplayCommand::SolidRect { rect, color },
            z_index,
        ));
    }

    /// Add text
    pub fn push_text(&mut self, text: String, x: f32, y: f32, color: Color, font_size: f32, z_index: i32) {
        self.push(DisplayItem::new(
            DisplayCommand::Text { text, x, y, color, font_size },
            z_index,
        ));
    }

    /// Add border
    pub fn push_border(
        &mut self,
        rect: Rect,
        widths: (f32, f32, f32, f32),
        colors: (Color, Color, Color, Color),
        z_index: i32,
    ) {
        self.push(DisplayItem::new(
            DisplayCommand::Border { rect, widths, colors },
            z_index,
        ));
    }

    /// Sort items by z-index for correct rendering order
    pub fn sort(&mut self) {
        self.items.sort_by_key(|item| item.z_index);
    }

    /// Get all items
    pub fn items(&self) -> &[DisplayItem] {
        &self.items
    }

    /// Clear the display list
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
