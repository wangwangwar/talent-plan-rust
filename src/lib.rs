#![deny(missing_docs)]
//! Key value store
use std::collections::HashMap;

/// Key value store struct
#[derive(Debug, PartialEq, Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Return the new instance
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Set the value of the string `key` to the `value`
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Get the value of the string `key`
    /// Get `Some(value)` when getting a existent key,
    /// get `None` when getting a non-existent key
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Remove the `key`
    /// Return `Some(value)` previously stored value when removing a existent key,
    /// return `None` when removing a non-existent key
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.map.remove(&key)
    }
}
