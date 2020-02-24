// #![deny(missing_docs)]
#![feature(seek_convenience)]

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
// #![feature(seek_convenience)]
#[macro_use]
extern crate failure_derive;

use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::str;
use std::{
    io::{Seek, SeekFrom}
};

// Some error type
pub type Result<T> = std::result::Result<T, Error>;

/// Object to set, get, and remove key value pairs
pub struct KvStore {
    index: HashMap<String, u64>,
    file: File,
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

        let position = self.log(c)?;

        self.index.insert(key, position);

        Ok(())
    }

    /// Get a previously set value for a key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let position;

        match self.index.get(&key) {
            Some(value) => { position = value},
            None => { return Ok(None) }
        };

        let mut line = String::new();
        let mut reader = BufReader::new(&self.file);

        reader.seek(SeekFrom::Start(*position))?;
        reader.read_line(&mut line)?;

        line = line.replace("\n", "");

        let command: Command = serde_json::from_str(&line)?;

        match command {
            Command::Set { key: _, value } => Ok(Some(value)),
            Command::Remove { key: _ } => Err(Error::from(KeyNotFound)),
        }
    }

    /// Remove a previously set key value
    pub fn remove(&mut self, key: String) -> Result<()> {
        let c = Command::Remove { key: key.clone() };

        self.log(c)?;

        match self.index.remove(&key) {
            None => Err(Error::from(KeyNotFound)),
            Some(_value) => Ok(()),
        }
    }

    fn log(&mut self, command: Command) -> Result<u64> {
        let position = self.file.stream_len()?;
        let mut s = serde_json::to_string(&command)?;

        s.push_str("\n");

        self.file.write_all(&s.into_bytes())?;

        Ok(position)
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path = path.into();
        path.push("log");

        let mut f = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)
            .unwrap();

        let file_len = f.stream_len()?;
        let mut index = HashMap::new();
        let mut position = 0;
        let mut reader = BufReader::new(&f);
        let mut line = String::new();

        reader.seek(SeekFrom::Start(position))?;
        reader.read_line(&mut line)?;

        while position < file_len {
            let command: Command = serde_json::from_str(&line)?;

            match command {
                Command::Set { key, value: _ } => index.insert(key, position),
                Command::Remove { key } => index.remove(&key),
            };

            position += line.len() as u64;

            line = String::new();

            reader.seek(SeekFrom::Start(position))?;
            reader.read_line(&mut line)?;
        }


        Ok(KvStore {
            index: index,
            file: f,
        })
    }
}
