//! Input state management

use crate::ui::events::Modifiers;

/// Current input state
#[derive(Debug, Default)]
pub struct InputState {
    /// Mouse position
    pub mouse_x: f32,
    pub mouse_y: f32,
    /// Mouse button states
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    /// Keyboard modifiers
    pub modifiers: Modifiers,
    /// Currently focused element (for keyboard input)
    pub focused_element: Option<u64>,
    /// Text input buffer
    pub text_buffer: String,
    /// Scroll position
    pub scroll_x: f32,
    pub scroll_y: f32,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update mouse position
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    /// Set mouse button state
    pub fn set_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.left_button = pressed,
            MouseButton::Right => self.right_button = pressed,
            MouseButton::Middle => self.middle_button = pressed,
        }
    }

    /// Update scroll position
    pub fn scroll(&mut self, dx: f32, dy: f32) {
        self.scroll_x += dx;
        self.scroll_y += dy;
        // Clamp scroll position
        self.scroll_y = self.scroll_y.max(0.0);
    }

    /// Reset scroll position
    pub fn reset_scroll(&mut self) {
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
    }

    /// Clear text buffer
    pub fn clear_text(&mut self) {
        self.text_buffer.clear();
    }

    /// Append to text buffer
    pub fn append_text(&mut self, text: &str) {
        self.text_buffer.push_str(text);
    }
}

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
