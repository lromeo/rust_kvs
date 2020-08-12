// #![deny(missing_docs)]
#![feature(seek_convenience)]

//! # PNA Rust Project - Key Value KvStore
//!
//! # Examples
//! ```
//! use kvs::{KvStore, KvsEngine};
//! use tempfile::TempDir;
//! let temp_dir = TempDir::new().unwrap();
//! let mut store = KvStore::open(temp_dir.path()).unwrap();
//!
//! store.set("key1".to_owned(), "value1".to_owned());
//! assert_eq!(store.get("key1".to_owned()).unwrap(), Some("value1".to_owned()));
//!
//! store.remove("key1".to_owned());
//! assert_eq!(store.get("key1".to_owned()).unwrap(), None);
//! ```
//!
// #![feature(seek_convenience)]
#[macro_use]
extern crate failure_derive;
extern crate log;
extern crate sled;

mod engines;

pub use engines::{KvStore, KvsEngine, SledKvStore};

use failure::Error;
use log::{Level, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::str;

// Some error type
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub value: Option<String>,
    pub error: Option<String>,
}

impl Response {
    pub fn new(result: Result<Option<String>>) -> Response {
        match result {
            Ok(value) => Response { value, error: None },
            Err(error) => Response {
                value: None,
                error: Some(error.to_string()),
            },
        }
    }

    pub fn is_error(&self) -> bool {
        match self.error {
            Some(_) => true,
            None => false,
        }
    }
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{} - {}", record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

#[derive(Fail, Debug)]
#[fail(display = "Key not found")]
pub struct KeyNotFound;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}
