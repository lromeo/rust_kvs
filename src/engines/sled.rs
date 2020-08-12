use super::KvsEngine;

use crate::{KeyNotFound, Result};
use failure::Error;
use std::path::PathBuf;
use std::str;

pub struct SledKvStore {
    db: sled::Db,
}

impl SledKvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<SledKvStore> {
        let path = path.into();

        let db = sled::open(path.join("current_sled_log"))?;

        Ok(SledKvStore { db })
    }
}

impl KvsEngine for SledKvStore {
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let value = self.db.get(&key)?;

        match value {
            Some(vec) => Ok(Some(str::from_utf8(&vec).unwrap().to_owned())),
            None => Ok(None),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key.into_bytes(), value.into_bytes())?;

        self.db.flush()?;

        Ok(())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let result = self.db.remove(&key)?;

        self.db.flush()?;

        match result {
            Some(_) => Ok(()),
            None => Err(Error::from(KeyNotFound)),
        }
    }
}
