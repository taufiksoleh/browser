//! Tab management
//!
//! Each tab represents an isolated browsing context with its own:
//! - DOM tree
//! - Style context
//! - Layout tree
//! - Render state

use crate::dom::Document;
use crate::css::StyleContext;
use crate::layout::LayoutTree;
use crate::network::NetworkClient;
use url::Url;

/// Represents a browser tab with isolated context
pub struct Tab {
    /// Unique tab identifier
    pub id: u64,
    /// Current URL
    pub url: Option<Url>,
    /// Page title
    pub title: String,
    /// DOM document
    pub document: Option<Document>,
    /// Style context for CSS
    pub style_context: StyleContext,
    /// Layout tree
    pub layout_tree: Option<LayoutTree>,
    /// Loading state
    pub loading: bool,
    /// Network client for this tab
    network: NetworkClient,
}

impl Tab {
    /// Create a new empty tab
    pub fn new(id: u64) -> Self {
        Self {
            id,
            url: None,
            title: String::from("New Tab"),
            document: None,
            style_context: StyleContext::new(),
            layout_tree: None,
            loading: false,
            network: NetworkClient::new(),
        }
    }

    /// Navigate to a URL
    pub async fn navigate(&mut self, url: &str) -> crate::browser::Result<()> {
        self.loading = true;

        // Parse URL
        let parsed_url = if url.starts_with("http://") || url.starts_with("https://") {
            Url::parse(url).map_err(|e| crate::browser::BrowserError::Parse(e.to_string()))?
        } else {
            Url::parse(&format!("https://{}", url))
                .map_err(|e| crate::browser::BrowserError::Parse(e.to_string()))?
        };

        self.url = Some(parsed_url.clone());

        // Fetch the page
        let response = self.network.fetch(&parsed_url).await?;

        // Parse HTML into DOM
        let document = crate::dom::parse_html(&response.body);

        // Extract title
        self.title = document.get_title().unwrap_or_else(|| parsed_url.host_str().unwrap_or("").to_string());

        self.document = Some(document);
        self.loading = false;

        // Trigger style calculation and layout
        self.compute_style();
        self.compute_layout();

        Ok(())
    }

    /// Compute styles for the document
    pub fn compute_style(&mut self) {
        if let Some(ref mut doc) = self.document {
            self.style_context.apply_styles(doc);
        }
    }

    /// Compute layout
    pub fn compute_layout(&mut self) {
        if let Some(ref doc) = self.document {
            self.layout_tree = Some(LayoutTree::build(doc, &self.style_context));
        }
    }

    /// Get viewport dimensions
    pub fn viewport(&self) -> (u32, u32) {
        (1920, 1080) // Default, should be set from window
    }
}
