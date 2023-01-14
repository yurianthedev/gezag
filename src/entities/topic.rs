use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Topic {
    id: TopicId,
    name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct TopicId(pub Uuid);
