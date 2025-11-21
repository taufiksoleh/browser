//! Inline Layout Algorithm
//!
//! Implements CSS inline formatting context for text and inline elements.

use crate::layout::{LayoutTree, LayoutId, Rect};

/// Line box for inline layout
#[derive(Debug, Clone)]
pub struct LineBox {
    /// Bounding rectangle
    pub bounds: Rect,
    /// Fragments in this line
    pub fragments: Vec<InlineFragment>,
    /// Baseline position
    pub baseline: f32,
}

impl LineBox {
    pub fn new(y: f32, width: f32) -> Self {
        Self {
            bounds: Rect::new(0.0, y, width, 0.0),
            fragments: Vec::new(),
            baseline: 0.0,
        }
    }

    /// Add a fragment to this line
    pub fn add_fragment(&mut self, fragment: InlineFragment) {
        // Update line height
        self.bounds.height = self.bounds.height.max(fragment.bounds.height);
        self.fragments.push(fragment);
    }

    /// Calculate remaining space in line
    pub fn remaining_width(&self) -> f32 {
        let used: f32 = self.fragments.iter().map(|f| f.bounds.width).sum();
        self.bounds.width - used
    }
}

/// Inline fragment (piece of inline content)
#[derive(Debug, Clone)]
pub struct InlineFragment {
    /// Layout node this belongs to
    pub layout_id: LayoutId,
    /// Bounding rectangle
    pub bounds: Rect,
    /// Text content (for text fragments)
    pub text: Option<String>,
    /// Fragment type
    pub fragment_type: InlineFragmentType,
}

/// Type of inline fragment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineFragmentType {
    Text,
    InlineElement,
    InlineBlock,
    Image,
}

/// Inline formatting context
pub struct InlineContext {
    /// Available width
    pub width: f32,
    /// Line boxes
    pub lines: Vec<LineBox>,
    /// Current line
    current_line: usize,
    /// Current x position
    cursor_x: f32,
    /// Start y position
    start_y: f32,
}

impl InlineContext {
    pub fn new(width: f32, start_y: f32) -> Self {
        let mut ctx = Self {
            width,
            lines: Vec::new(),
            current_line: 0,
            cursor_x: 0.0,
            start_y,
        };
        ctx.new_line();
        ctx
    }

    /// Start a new line
    fn new_line(&mut self) {
        let y = if let Some(last) = self.lines.last() {
            last.bounds.y + last.bounds.height
        } else {
            self.start_y
        };
        self.lines.push(LineBox::new(y, self.width));
        self.current_line = self.lines.len() - 1;
        self.cursor_x = 0.0;
    }

    /// Add a text fragment
    pub fn add_text(&mut self, layout_id: LayoutId, text: &str, font_size: f32) {
        // Simple text measurement (would use actual font metrics in production)
        let char_width = font_size * 0.6;
        let line_height = font_size * 1.2;

        for word in text.split_whitespace() {
            let word_width = word.len() as f32 * char_width;

            // Check if word fits on current line
            if self.cursor_x + word_width > self.width && self.cursor_x > 0.0 {
                self.new_line();
            }

            let fragment = InlineFragment {
                layout_id,
                bounds: Rect::new(self.cursor_x, 0.0, word_width, line_height),
                text: Some(word.to_string()),
                fragment_type: InlineFragmentType::Text,
            };

            if let Some(line) = self.lines.get_mut(self.current_line) {
                line.add_fragment(fragment);
            }

            self.cursor_x += word_width + char_width; // Add space width
        }
    }

    /// Add an inline element
    pub fn add_inline_element(&mut self, layout_id: LayoutId, width: f32, height: f32) {
        // Check if element fits on current line
        if self.cursor_x + width > self.width && self.cursor_x > 0.0 {
            self.new_line();
        }

        let fragment = InlineFragment {
            layout_id,
            bounds: Rect::new(self.cursor_x, 0.0, width, height),
            text: None,
            fragment_type: InlineFragmentType::InlineElement,
        };

        if let Some(line) = self.lines.get_mut(self.current_line) {
            line.add_fragment(fragment);
        }

        self.cursor_x += width;
    }

    /// Get total height of all lines
    pub fn total_height(&self) -> f32 {
        self.lines.iter().map(|l| l.bounds.height).sum()
    }

    /// Finalize and return line boxes
    pub fn finalize(self) -> Vec<LineBox> {
        self.lines
    }
}
