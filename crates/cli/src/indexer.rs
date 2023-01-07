pub mod local;

use crate::repositories::ResourceRepository;

pub trait Indexer: ResourceRepository {}
