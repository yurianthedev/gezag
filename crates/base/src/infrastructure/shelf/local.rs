use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::domain::error::GenericError;
use crate::domain::repository::Add;

pub struct Local {
    index_file: File,
}

impl Local {
    pub fn new<T: Clone + for<'a> Deserialize<'a> + Serialize + Default>(
        index_path: &Path,
    ) -> Result<Self, GenericError> {
        let index_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(index_path)?;

        let mut local = Self { index_file };
        if local.read_toml::<Index<T>>().is_err() {
            local.write_toml(Index::<T>::default())?;
        };

        Ok(local)
    }

    fn read_toml<T: Clone + Serialize + for<'b> Deserialize<'b>>(
        &mut self,
    ) -> Result<T, GenericError> {
        let mut contents = String::new();
        self.index_file.read_to_string(&mut contents)?;

        Ok(toml::from_str::<T>(&contents)?)
    }

    fn write_toml<T: Serialize>(&mut self, index: Index<T>) -> Result<(), GenericError> {
        self.index_file
            .write_all(toml::to_string_pretty(&index)?.as_bytes())?;
        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Index<T> {
    resources: Vec<T>,
}

impl<Item> Add<Item> for Local
where
    Item: Clone + Serialize + for<'b> Deserialize<'b>,
{
    fn add(&mut self, item: Item) -> Result<(), crate::domain::error::GenericError> {
        let mut index: Index<Item> = self.read_toml()?;
        index.resources.push(item);
        self.write_toml(index)?;

        Ok(())
    }
}
