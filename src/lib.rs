#![deny(missing_docs)]

//! # PNA Rust Project - Key Value KvStore
//!
//! # Examples
//! ```
//! use kvs::KvStore;
//! let mut store = KvStore::new();
//!
//! store.set("key1".to_owned(), "value1".to_owned());
//! assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
//!
//! store.remove("key1".to_owned());
//! assert_eq!(store.get("key1".to_owned()), None);
//! ```
//!

use std::collections::HashMap;

/// Object to set, get, and remove key value pairs
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// Construct a new KvStore with no stored key value pairs
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Set a key value pair to be accessible later
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get a previously set value for a key
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Remove a previously set key value
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
