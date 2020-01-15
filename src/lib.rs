// #![deny(missing_docs)]

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
#[macro_use]
extern crate failure_derive;

use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;

// Some error type
pub type Result<T> = std::result::Result<T, Error>;

/// Object to set, get, and remove key value pairs
pub struct KvStore {
    data: HashMap<String, String>,
    writer: BufWriter<File>,
}

#[derive(Fail, Debug)]
#[fail(display = "Key not found")]
pub struct KeyNotFound;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl KvStore {
    /// Set a key value pair to be accessible later
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let c = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };

        self.log(c);

        self.data.insert(key, value);

        Ok(())
    }

    /// Get a previously set value for a key
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.data.get(&key).cloned())
    }

    /// Remove a previously set key value
    pub fn remove(&mut self, key: String) -> Result<()> {
        let c = Command::Remove { key: key.clone() };

        self.log(c);

        match self.data.remove(&key) {
            None => Err(Error::from(KeyNotFound)),
            Some(_value) => Ok(()),
        }
    }

    fn log(&mut self, command: Command) {
        let mut s = serde_json::to_string(&command).unwrap();

        s.push_str("\n");

        self.writer.write(&s.into_bytes()).unwrap();
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path = path.into();
        path.push("log");

        let f = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)
            .unwrap();

        // Load previous data from log
        let mut data = HashMap::new();

        for line in BufReader::new(&f).lines() {
            let command: Command = serde_json::from_str(&line.unwrap())?;

            match command {
                Command::Set { key, value } => data.insert(key, value),
                Command::Remove { key } => data.remove(&key),
            };
        }

        Ok(KvStore {
            data: data,
            writer: BufWriter::new(f),
        })
    }
}
