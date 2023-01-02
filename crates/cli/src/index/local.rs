use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use crate::resources::{Add, Remove, Resource};

use super::Index;

pub struct Config {
    index_location: String,
}

pub struct Indexer {
    index_path: PathBuf,
}

#[derive(Debug)]
pub enum UpdateError {
    Io(io::Error),
    JsonSer(serde_json::Error),
}

impl From<io::Error> for UpdateError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for UpdateError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonSer(value)
    }
}

impl Indexer {
    pub fn new(location: &str) -> Result<Self, io::Error> {
        let path = Path::new(location).to_owned();
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        Ok(Indexer { index_path: path })
    }

    fn update<F>(&self, updt: F) -> Result<(), UpdateError>
    where
        F: FnOnce(&mut Index),
    {
        let content = fs::read_to_string(&self.index_path)?;
        let mut index: Index = serde_json::from_str(&content).unwrap_or_default();

        updt(&mut index);

        fs::write(
            &self.index_path,
            serde_json::to_string_pretty(&index)?.as_bytes(),
        )?;
        Ok(())
    }
}

impl Add<Resource> for Indexer {
    type S = ();
    type E = UpdateError;

    fn add(&self, res: Resource) -> Result<Self::S, Self::E> {
        self.update(|index| index.resources.push(res))
    }
}

impl Remove for Indexer {
    type K = uuid::Uuid;
    type S = ();
    type E = UpdateError;

    fn remove(&self, key: Self::K) -> Result<Self::S, Self::E> {
        self.update(|index| {
            index.resources.retain(|res| res.id == key);
        })
    }
}
