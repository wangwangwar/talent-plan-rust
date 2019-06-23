#![deny(missing_docs)]
//! Key value store
use std::collections::HashMap;
use std::path::Path;
use std::io;
use std::result;

/// Key value store struct
#[derive(Debug, PartialEq, Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

/// Custom error type
#[derive(Debug)]
pub enum Error {
    /// IO Error
    Io(io::Error)
}

/// Custom result type
pub type Result<T> = result::Result<T, Error>;

impl KvStore {
    /// Return the new instance
    pub fn open(_path: &Path) -> Result<Self> {
        Ok(KvStore {
            map: HashMap::new(),
        })
    }

    /// Set the value of the string `key` to the `value`
    /// Return `Ok` if success,
    /// return `Err` if failure
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        panic!()
    }

    /// Get the value of the string `key`
    /// Return `Ok(Some)` when getting a existent key,
    /// return `Ok(None)` when getting a non-existent key,
    /// return `Err` when error
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.map.get(&key).cloned();
        panic!()
    }

    /// Remove the `key`
    /// Return `Ok(Some)` previously stored value when removing a existent key,
    /// return `Ok(None)` when removing a non-existent key,
    /// return `Err` when error
    pub fn remove(&mut self, key: String) -> Result<Option<String>> {
        self.map.remove(&key);
        panic!()
    }
}
