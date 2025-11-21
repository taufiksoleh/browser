//! User Interface Layer
//!
//! Provides:
//! - Window management with winit
//! - Event handling
//! - Input processing
//! - UI components (address bar, tabs, etc.)

mod events;
mod input;
mod window;

pub use events::BrowserEvent;
pub use input::InputState;
pub use window::{Window, WindowConfig};
