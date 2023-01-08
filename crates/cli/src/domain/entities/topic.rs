use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Topic {
    id: Uuid,
    name: String,
}
