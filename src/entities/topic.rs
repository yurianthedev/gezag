use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default, Builder, Clone)]
pub struct Topic {
    pub id: TopicId,
    name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct TopicId(pub Uuid);
