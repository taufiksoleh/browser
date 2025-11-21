//! Window management using winit

use crate::browser::Browser;
use crate::render::Renderer;
use crate::ui::input::MouseButton;
use crate::ui::{BrowserEvent, InputState};
use parking_lot::RwLock;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, MouseButton as WinitMouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{WindowAttributes, WindowId},
};

/// Window configuration for creating windows
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    /// Create the event loop (window is created later in resumed callback)
    pub fn create_event_loop() -> Result<EventLoop<()>, String> {
        let event_loop =
            EventLoop::new().map_err(|e| format!("Failed to create event loop: {}", e))?;
        event_loop.set_control_flow(ControlFlow::Wait);
        Ok(event_loop)
    }

    /// Run the event loop
    pub fn run(
        event_loop: EventLoop<()>,
        browser: Arc<RwLock<Browser>>,
        config: WindowConfig,
    ) -> Result<(), String> {
        let mut app = BrowserApp::new(browser, config);
        event_loop
            .run_app(&mut app)
            .map_err(|e| format!("Event loop error: {}", e))
    }
}

/// Browser window wrapper (used after window is created)
pub struct Window;

impl Window {
    /// Get inner window size from a winit window
    pub fn inner_size_from(window: &winit::window::Window) -> (u32, u32) {
        let size = window.inner_size();
        (size.width, size.height)
    }
}

/// Browser application handler
struct BrowserApp {
    browser: Arc<RwLock<Browser>>,
    renderer: Option<Renderer>,
    window: Option<Arc<winit::window::Window>>,
    input: InputState,
    config: WindowConfig,
}

impl BrowserApp {
    fn new(browser: Arc<RwLock<Browser>>, config: WindowConfig) -> Self {
        Self {
            browser,
            renderer: None,
            window: None,
            input: InputState::new(),
            config,
        }
    }

    fn handle_event(&mut self, event: BrowserEvent) {
        match event {
            BrowserEvent::Navigate(url) => {
                log::info!("Navigating to: {}", url);
                // Navigation would be handled asynchronously
            }
            BrowserEvent::Scroll { dx, dy } => {
                self.input.scroll(dx, dy);
            }
            BrowserEvent::Click { x, y } => {
                log::debug!("Click at ({}, {})", x, y);
            }
            BrowserEvent::Resize { width, height } => {
                if let Some(ref mut renderer) = self.renderer {
                    renderer.resize((width, height));
                }
            }
            BrowserEvent::Quit => {
                // Handled in event loop
            }
            _ => {}
        }
    }

    fn render(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            // Get display list from browser
            // For now, just render an empty display list
            let display_list = crate::render::DisplayList::new();
            if let Err(e) = renderer.render(&display_list) {
                log::error!("Render error: {}", e);
            }
        }
    }
}

impl ApplicationHandler for BrowserApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = WindowAttributes::default()
                .with_title(&self.config.title)
                .with_inner_size(LogicalSize::new(
                    self.config.width as f64,
                    self.config.height as f64,
                ))
                .with_min_inner_size(LogicalSize::new(400.0, 300.0));

            match event_loop.create_window(window_attrs) {
                Ok(window) => {
                    let window = Arc::new(window);
                    self.window = Some(window.clone());

                    // Create renderer now that we have a real window
                    match Renderer::new_with_window(window.clone()) {
                        Ok(renderer) => {
                            self.renderer = Some(renderer);
                            log::info!("Window and renderer created successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to create renderer: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to create window: {}", e);
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                self.handle_event(BrowserEvent::Resize { width, height });
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input
                    .set_mouse_position(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                let btn = match button {
                    WinitMouseButton::Left => MouseButton::Left,
                    WinitMouseButton::Right => MouseButton::Right,
                    WinitMouseButton::Middle => MouseButton::Middle,
                    _ => return,
                };
                self.input.set_mouse_button(btn, pressed);

                if pressed && btn == MouseButton::Left {
                    self.handle_event(BrowserEvent::Click {
                        x: self.input.mouse_x,
                        y: self.input.mouse_y,
                    });
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                    winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                self.handle_event(BrowserEvent::Scroll { dx, dy });
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    // Handle keyboard shortcuts
                    // Ctrl+T for new tab, Ctrl+W for close tab, etc.
                }
            }
            WindowEvent::Focused(focused) => {
                self.handle_event(BrowserEvent::Focus(focused));
            }
            _ => {}
        }

        // Request redraw after handling events
        if let Some(ref window) = self.window {
            window.request_redraw();
        }
    }
}
