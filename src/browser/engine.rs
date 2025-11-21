//! Main Browser Engine
//!
//! Coordinates the rendering pipeline:
//! Parse → Style → Layout → Paint → Composite

use crate::browser::{BrowserError, Result, Tab};
use crate::ui::{Window, WindowConfig};
use log::info;
use parking_lot::RwLock;
use std::sync::Arc;

/// Main browser instance
pub struct Browser {
    /// Active tabs
    tabs: Vec<Tab>,
    /// Currently active tab index
    active_tab: usize,
    /// Tab ID counter
    next_tab_id: u64,
}

impl Browser {
    /// Create a new browser instance
    pub fn new() -> Self {
        let mut browser = Self {
            tabs: Vec::new(),
            active_tab: 0,
            next_tab_id: 0,
        };
        // Create initial tab
        browser.new_tab();
        browser
    }

    /// Run the browser event loop
    pub fn run() -> Result<()> {
        info!("Initializing browser engine...");

        let browser = Arc::new(RwLock::new(Self::new()));

        // Create event loop (window and renderer are created in the resumed callback)
        let event_loop =
            Window::create_event_loop().map_err(|e| BrowserError::Window(e.to_string()))?;

        let config = WindowConfig {
            title: "Browser".to_string(),
            width: 1280,
            height: 720,
        };

        info!("Browser engine initialized, starting event loop...");

        // Run the event loop
        Window::run(event_loop, browser, config).map_err(|e| BrowserError::Window(e.to_string()))
    }

    /// Create a new tab
    pub fn new_tab(&mut self) -> usize {
        let tab = Tab::new(self.next_tab_id);
        self.next_tab_id += 1;
        self.tabs.push(tab);
        let idx = self.tabs.len() - 1;
        self.active_tab = idx;
        idx
    }

    /// Close a tab
    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() > 1 && index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    /// Get active tab
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab)
    }

    /// Get active tab mutably
    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab)
    }

    /// Switch to a specific tab
    pub fn switch_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }

    /// Get all tabs
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Navigate active tab to URL
    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        if let Some(tab) = self.active_tab_mut() {
            tab.navigate(url).await
        } else {
            Ok(())
        }
    }
}

impl Default for Browser {
    fn default() -> Self {
        Self::new()
    }
}
