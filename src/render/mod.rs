//! GPU-Accelerated Rendering Pipeline
//!
//! Uses wgpu for cross-platform GPU rendering:
//! - Display list generation
//! - Tiled rasterization
//! - Hardware compositing

mod display_list;
mod gpu;
mod paint;
mod renderer;

pub use display_list::{DisplayCommand, DisplayItem, DisplayList};
pub use renderer::Renderer;
