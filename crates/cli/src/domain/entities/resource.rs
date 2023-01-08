use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use super::topic::Topic;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub id: ResourceId,
    pub kind: Kind,
    pub topics: HashSet<Topic>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceId(Uuid);

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceIndeces {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceKeys {}

pub struct ResourceBuilder {}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum Kind {
    Book { title: String, author: String },
}
