use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use super::topics::Topic;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub metadata: Kind,
    pub topics: HashSet<Topic>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Kind {
    Book { title: String, author: String },
}

pub trait Add<T> {
    type S;
    type E;

    fn add(&self, res: T) -> Result<Self::S, Self::E>;
}

pub trait Remove {
    type K;
    type S;
    type E;

    fn remove(&self, key: Self::K) -> Result<Self::S, Self::E>;
}

pub trait ResourceRepository: Add<Resource> + Remove {}
