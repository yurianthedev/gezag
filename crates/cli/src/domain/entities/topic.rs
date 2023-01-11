use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Topic {
    id: TopicId,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TopicId(pub Uuid);
