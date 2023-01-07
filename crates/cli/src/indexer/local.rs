use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::{
    entities::{
        self,
        resources::{Indeces, Keys, Resource},
        topics::Topic,
        Key,
    },
    repositories::{
        repository::{Add, List, ListBy, Remove},
        ResourceRepository,
    },
};

/// It should store:
/// Data of resources.
/// Data of topics.
/// Relations between resources and topics.
#[derive(Serialize, Deserialize, Default)]
pub struct Index {
    resources: Vec<Resource>,
    topics: Vec<Topic>,
}

pub struct Config {
    index_location: String,
}

pub struct Indexer {
    index_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("error parsing json")]
    JsonSer(#[from] serde_json::Error),
}

impl Indexer {
    pub fn new(location: &str) -> Result<Self, io::Error> {
        let path = Path::new(location).to_owned();
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?; // Opens or creates (atomically) the file containing the index.

        Ok(Indexer { index_path: path })
    }

    fn read(&self) -> Result<Index, UpdateError> {
        let content = fs::read_to_string(&self.index_path)?;
        let index: Index = serde_json::from_str(&content).unwrap_or_default();

        Ok(index)
    }

    fn write(&self, index: &Index) -> Result<(), UpdateError> {
        fs::write(
            &self.index_path,
            serde_json::to_string_pretty(index)?.as_bytes(),
        )?;

        Ok(())
    }

    fn update<F>(&self, updt: F) -> Result<(), UpdateError>
    where
        F: FnOnce(&mut Index),
    {
        let mut index: Index = self.read()?;
        updt(&mut index);
        self.write(&index)?;

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

impl Remove<Resource> for Indexer {
    type K = Keys;
    type S = ();
    type E = UpdateError;

    fn remove(&self, key: Self::K) -> Result<Self::S, Self::E> {
        self.update(|index| {
            index.resources.retain(|res| !key.is(res));
        })
    }
}

impl List<Resource> for Indexer {
    type S = Vec<Resource>;
    type E = UpdateError;

    fn list(&self) -> Result<Self::S, Self::E> {
        Ok(self.read()?.resources)
    }
}

impl ListBy<Resource> for Indexer {
    type I = Indeces;
    type S = Vec<Resource>;
    type E = UpdateError;

    fn list_by(&self, index: &Self::I) -> Result<Self::S, Self::E> {
        Ok(self
            .read()?
            .resources
            .into_iter()
            .filter(|res| entities::Index::is_related(index, res))
            .collect())
    }
}

impl ResourceRepository for Indexer {}

impl super::Indexer for Indexer {}
