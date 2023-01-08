use crate::domain::{
    entities::resource::{Resource, ResourceBuilder, ResourceId, ResourceIndeces, ResourceKeys},
    repositories,
};

use super::Librarian;

impl repositories::ResourcesRepository for Librarian {
    fn add(&self, builder: ResourceBuilder) -> Result<ResourceId, anyhow::Error> {
        // self.update(|index| index.resources.push())
        todo!()
    }

    fn remove(&self, key: &ResourceKeys) -> Result<(), anyhow::Error> {
        // self.update(|index| {
        //     index.resources.retain(|res| !key.is(res));
        // });
        todo!()
    }

    fn list(&self) -> Result<Vec<Resource>, anyhow::Error> {
        Ok(self.read()?.resources)
    }

    fn list_by(&self, index: &ResourceIndeces) -> Result<Vec<Resource>, anyhow::Error> {
        // Ok(self
        //     .read()?
        //     .resources
        //     .into_iter()
        //     .filter(|res| entities::Index::is_related(index, res))
        //     .collect())
        todo!()
    }

    fn fetch(&self, key: &ResourceKeys) -> Result<Resource, anyhow::Error> {
        todo!()
    }
}
