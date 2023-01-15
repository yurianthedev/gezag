use thiserror::Error;
use uuid::Uuid;

use crate::{
    entities::{
        resource::{Resource, ResourceBuilder, ResourceId, ResourceIndeces, ResourceKeys},
        Indexable, Keyable,
    },
    repositories::Resources,
};

use super::Librarian;

impl Resources for Librarian {
    fn add(&self, builder: ResourceBuilder) -> Result<ResourceId, anyhow::Error> {
        let mut builder = builder;
        let id = ResourceId(Uuid::new_v4());
        builder.id(id.clone());
        let resource = builder.build()?;
        self.update(|index| index.resources.push(resource))?;
        Ok(id)
    }

    fn remove(&self, key: &ResourceKeys) -> Result<(), anyhow::Error> {
        self.update(|index| {
            index.resources.retain(|res| res.is(key));
        })?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Resource>, anyhow::Error> {
        Ok(self.read()?.resources)
    }

    fn list_by(&self, index: &ResourceIndeces) -> Result<Vec<Resource>, anyhow::Error> {
        Ok(self
            .read()?
            .resources
            .into_iter()
            .filter(|res| res.is_related_to(index))
            .collect())
    }

    fn fetch(&self, key: &ResourceKeys) -> Result<Resource, anyhow::Error> {
        if let Some(resource) = self.read()?.resources.into_iter().find(|res| res.is(key)) {
            Ok(resource)
        } else {
            Err(LibrarianError::NotFound(key.to_owned()))?
        }
    }
}
 

#[derive(Debug, Error)]
pub enum LibrarianError {
    #[error("not found")]
    NotFound(ResourceKeys),
}

