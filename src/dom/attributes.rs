//! Element attributes
//!
//! Uses FnvHashMap for fast attribute lookups with short keys.

use fnv::FnvHashMap;

/// Element attributes collection
#[derive(Debug, Clone, Default)]
pub struct Attributes {
    /// Attribute map (name -> value)
    attrs: FnvHashMap<String, String>,
}

impl Attributes {
    /// Create empty attributes
    pub fn new() -> Self {
        Self {
            attrs: FnvHashMap::default(),
        }
    }

    /// Create attributes with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            attrs: FnvHashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    /// Get an attribute value
    pub fn get(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(|s| s.as_str())
    }

    /// Set an attribute value
    pub fn set(&mut self, name: String, value: String) {
        self.attrs.insert(name, value);
    }

    /// Remove an attribute
    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.attrs.remove(name)
    }

    /// Check if attribute exists
    pub fn contains(&self, name: &str) -> bool {
        self.attrs.contains_key(name)
    }

    /// Get number of attributes
    pub fn len(&self) -> usize {
        self.attrs.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.attrs.is_empty()
    }

    /// Iterate over attributes
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.attrs.iter()
    }

    /// Get classes as iterator
    pub fn classes(&self) -> impl Iterator<Item = &str> {
        self.get("class")
            .map(|c| c.split_whitespace())
            .into_iter()
            .flatten()
    }

    /// Check if element has a specific class
    pub fn has_class(&self, class: &str) -> bool {
        self.classes().any(|c| c == class)
    }

    /// Get ID attribute
    pub fn id(&self) -> Option<&str> {
        self.get("id")
    }
}
