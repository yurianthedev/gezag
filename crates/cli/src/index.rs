pub mod local;

use serde::{Deserialize, Serialize};

use crate::entities::{resources::Resource, topics::Topic};

/// It should store:
/// Data of resources.
/// Data of topics.
/// Relations between resources and topics.
#[derive(Serialize, Deserialize, Default)]
pub struct Index {
    resources: Vec<Resource>,
    topics: Vec<Topic>,
}
