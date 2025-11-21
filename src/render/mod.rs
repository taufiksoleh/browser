//! GPU-Accelerated Rendering Pipeline
//!
//! Uses wgpu for cross-platform GPU rendering:
//! - Display list generation
//! - Tiled rasterization
//! - Hardware compositing

mod renderer;
mod display_list;
mod paint;
mod gpu;

pub use renderer::Renderer;
pub use display_list::{DisplayList, DisplayItem, DisplayCommand};
pub use paint::Painter;
