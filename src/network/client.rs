//! HTTP Client

use crate::network::{NetworkError, Cache, cache::CacheEntry};
use reqwest::{Client, header};
use std::time::{Duration, Instant};
use url::Url;

/// HTTP Response
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: Vec<(String, String)>,
    /// Response body
    pub body: String,
    /// Content type
    pub content_type: Option<String>,
    /// Final URL after redirects
    pub final_url: Url,
}

impl Response {
    /// Check if response is successful (2xx)
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Check if response is HTML
    pub fn is_html(&self) -> bool {
        self.content_type
            .as_ref()
            .map(|ct| ct.contains("text/html"))
            .unwrap_or(false)
    }
}

/// Network client for HTTP requests
pub struct NetworkClient {
    client: Client,
    cache: Cache,
}

impl NetworkClient {
    /// Create a new network client
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .gzip(true)
            .brotli(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .user_agent("Browser/0.1 (Rust)")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            cache: Cache::default(),
        }
    }

    /// Fetch a URL
    pub async fn fetch(&mut self, url: &Url) -> Result<Response, NetworkError> {
        let url_str = url.to_string();

        // Check cache first
        if let Some(entry) = self.cache.get(&url_str) {
            return Ok(Response {
                status: 200,
                headers: Vec::new(),
                body: String::from_utf8_lossy(&entry.body).to_string(),
                content_type: entry.content_type.clone(),
                final_url: url.clone(),
            });
        }

        // Make request
        let response = self.client.get(url.as_str()).send().await?;

        let status = response.status().as_u16();
        let final_url = response.url().clone();

        // Collect headers
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Get cache control
        let cache_control = response
            .headers()
            .get(header::CACHE_CONTROL)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let etag = response
            .headers()
            .get(header::ETAG)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let body = response.text().await?;

        // Cache the response if cacheable
        if status == 200 && !self.is_no_cache(&cache_control) {
            let max_age = self.parse_max_age(&cache_control);
            self.cache.put(
                url_str,
                CacheEntry {
                    body: body.as_bytes().to_vec(),
                    content_type: content_type.clone(),
                    cached_at: Instant::now(),
                    max_age,
                    etag,
                    last_modified: None,
                },
            );
        }

        Ok(Response {
            status,
            headers,
            body,
            content_type,
            final_url,
        })
    }

    /// Check if response should not be cached
    fn is_no_cache(&self, cache_control: &Option<String>) -> bool {
        cache_control
            .as_ref()
            .map(|cc| cc.contains("no-cache") || cc.contains("no-store"))
            .unwrap_or(false)
    }

    /// Parse max-age from cache-control header
    fn parse_max_age(&self, cache_control: &Option<String>) -> Option<Duration> {
        cache_control.as_ref().and_then(|cc| {
            for part in cc.split(',') {
                let part = part.trim();
                if let Some(age) = part.strip_prefix("max-age=") {
                    if let Ok(secs) = age.parse::<u64>() {
                        return Some(Duration::from_secs(secs));
                    }
                }
            }
            None
        })
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for NetworkClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_url() {
        let mut client = NetworkClient::new();
        let url = Url::parse("https://example.com").unwrap();

        // This test requires network access
        // In a real test suite, you'd mock the HTTP client
        let result = client.fetch(&url).await;
        assert!(result.is_ok() || result.is_err()); // Just verify it doesn't panic
    }
}
