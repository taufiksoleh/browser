//! Browser events

/// Browser-specific events
#[derive(Debug, Clone)]
pub enum BrowserEvent {
    /// Navigate to URL
    Navigate(String),
    /// Go back in history
    Back,
    /// Go forward in history
    Forward,
    /// Reload current page
    Reload,
    /// Stop loading
    Stop,
    /// New tab
    NewTab,
    /// Close tab
    CloseTab(usize),
    /// Switch to tab
    SwitchTab(usize),
    /// Scroll by amount
    Scroll { dx: f32, dy: f32 },
    /// Click at position
    Click { x: f32, y: f32 },
    /// Key press
    KeyPress { key: String, modifiers: Modifiers },
    /// Text input
    TextInput(String),
    /// Window resized
    Resize { width: u32, height: u32 },
    /// Window focused
    Focus(bool),
    /// Quit browser
    Quit,
}

/// Keyboard modifiers
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool, // Cmd on Mac, Win on Windows
}

impl Modifiers {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn with_ctrl() -> Self {
        Self {
            ctrl: true,
            ..Default::default()
        }
    }
}
