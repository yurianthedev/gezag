use crate::entities::resource::{
    Resource, ResourceBuilder, ResourceId, ResourceIndeces, ResourceKeys,
};

pub trait Resources {
    fn add(&self, builder: ResourceBuilder) -> Result<ResourceId, anyhow::Error>;
    fn remove(&self, key: &ResourceKeys) -> Result<(), anyhow::Error>;
    fn list(&self) -> Result<Vec<Resource>, anyhow::Error>;
    fn list_by(&self, index: &ResourceIndeces) -> Result<Vec<Resource>, anyhow::Error>;
    fn fetch(&self, key: &ResourceKeys) -> Result<Resource, anyhow::Error>;
}
