//! User Interface Layer
//!
//! Provides:
//! - Window management with winit
//! - Event handling
//! - Input processing
//! - UI components (address bar, tabs, etc.)

mod window;
mod input;
mod events;

pub use window::Window;
pub use input::InputState;
pub use events::BrowserEvent;
