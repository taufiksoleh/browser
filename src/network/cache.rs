//! HTTP Cache implementation

use fnv::FnvHashMap;
use std::time::{Duration, Instant};

/// Cached response entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Response body
    pub body: Vec<u8>,
    /// Content type
    pub content_type: Option<String>,
    /// When the entry was cached
    pub cached_at: Instant,
    /// Max age in seconds
    pub max_age: Option<Duration>,
    /// ETag for validation
    pub etag: Option<String>,
    /// Last-Modified header
    pub last_modified: Option<String>,
}

impl CacheEntry {
    /// Check if entry is still fresh
    pub fn is_fresh(&self) -> bool {
        if let Some(max_age) = self.max_age {
            self.cached_at.elapsed() < max_age
        } else {
            // Default: 5 minutes
            self.cached_at.elapsed() < Duration::from_secs(300)
        }
    }
}

/// Simple HTTP cache
pub struct Cache {
    entries: FnvHashMap<String, CacheEntry>,
    max_size: usize,
    current_size: usize,
}

impl Cache {
    /// Create a new cache with max size in bytes
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: FnvHashMap::default(),
            max_size,
            current_size: 0,
        }
    }

    /// Get a cached entry
    pub fn get(&self, url: &str) -> Option<&CacheEntry> {
        self.entries.get(url).filter(|e| e.is_fresh())
    }

    /// Store a response in cache
    pub fn put(&mut self, url: String, entry: CacheEntry) {
        let entry_size = entry.body.len();

        // Evict if necessary
        while self.current_size + entry_size > self.max_size && !self.entries.is_empty() {
            self.evict_oldest();
        }

        if let Some(old) = self.entries.insert(url, entry) {
            self.current_size -= old.body.len();
        }
        self.current_size += entry_size;
    }

    /// Evict the oldest entry
    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self
            .entries
            .iter()
            .min_by_key(|(_, v)| v.cached_at)
            .map(|(k, _)| k.clone())
        {
            if let Some(entry) = self.entries.remove(&oldest_key) {
                self.current_size -= entry.body.len();
            }
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_size = 0;
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.current_size
    }

    /// Get number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for Cache {
    fn default() -> Self {
        // 50MB default cache
        Self::new(50 * 1024 * 1024)
    }
}
