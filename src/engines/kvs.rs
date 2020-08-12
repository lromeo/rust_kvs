use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::PathBuf;

use failure::Error;

use super::KvsEngine;
use crate::{Command, KeyNotFound, Result};

pub struct KvStore {
    index: HashMap<String, u64>,
    file: File,
    path: PathBuf,
}

impl KvsEngine for KvStore {
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let position;

        match self.index.get(&key) {
            Some(value) => position = value,
            None => return Ok(None),
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
            _ => panic!(),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        let trigger_compaction = self.index.contains_key(&key);

        let c = Command::Set {
            key: key.clone(),
            value,
        };

        let position = self.log(c)?;

        self.index.insert(key, position);

        if trigger_compaction {
            self.compaction()?;
        }

        Ok(())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let c = Command::Remove { key: key.clone() };

        self.log(c)?;

        match self.index.remove(&key) {
            None => Err(Error::from(KeyNotFound)),
            Some(_value) => Ok(()),
        }?;

        self.compaction()
    }
}

impl KvStore {
    fn log(&mut self, command: Command) -> Result<u64> {
        let position = self.file.stream_len()?;
        let mut s = serde_json::to_string(&command)?;

        s.push_str("\n");

        self.file.write_all(&s.into_bytes())?;

        Ok(position)
    }

    pub fn compaction(&mut self) -> Result<()> {
        let next_path = self.path.clone().join("next_log");
        let current_path = self.path.clone().join("current_log");

        let mut next_index = HashMap::new();
        let mut next_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&next_path)
            .unwrap();

        next_file.set_len(0)?;

        let mut reader = BufReader::new(&self.file);

        for (key, position) in &self.index {
            let mut line = String::new();

            reader.seek(SeekFrom::Start(*position))?;
            reader.read_line(&mut line)?;

            next_index.insert(key.clone(), next_file.stream_len()?);
            next_file.write_all(&line.into_bytes())?;
        }

        self.file = next_file;
        self.index = next_index;

        fs::remove_file(&current_path)?;
        fs::rename(&next_path, &current_path)?;

        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();

        let mut f = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path.join("current_log"))
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
                _ => panic!(),
            };

            position += line.len() as u64;

            line = String::new();

            reader.seek(SeekFrom::Start(position))?;
            reader.read_line(&mut line)?;
        }

        Ok(KvStore {
            index,
            file: f,
            path,
        })
    }
}
