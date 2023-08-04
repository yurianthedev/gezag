pub mod resources;
pub mod topics;

use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::entities::{resource::Resource, topic::Topic};

#[derive(Serialize, Deserialize, Default)]
pub struct Registry {
    pub resources: Vec<Resource>,
    pub topics: Vec<Topic>,
}

pub struct Librarian {
    registry_path: PathBuf,
}

impl Librarian {
    pub fn new(location: &str) -> Result<Self, io::Error> {
        let default_registry_name = "registry.json";
        let path = Path::new(location).join(Path::new(default_registry_name));

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?; // Opens or creates (atomically) the file containing the index.

        Ok(Self {
            registry_path: path,
        })
    }

    pub(super) fn read(&self) -> Result<Registry, UpdateError> {
        let content = fs::read_to_string(&self.registry_path)?;
        let registry: Registry = serde_json::from_str(&content).unwrap_or_default();

        Ok(registry)
    }

    pub(super) fn write(&self, registry: &Registry) -> Result<(), UpdateError> {
        fs::write(
            &self.registry_path,
            serde_json::to_string_pretty(registry)?.as_bytes(),
        )?;

        Ok(())
    }

    pub(super) fn update<F>(&self, updt: F) -> Result<(), UpdateError>
    where
        F: FnOnce(&mut Registry),
    {
        let mut registry: Registry = self.read()?;
        updt(&mut registry);
        self.write(&registry)?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("error parsing json")]
    JsonSer(#[from] serde_json::Error),
}
