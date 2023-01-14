use anyhow::Result;

use crate::entities::topic::{Topic, TopicBuilder, TopicId};

pub trait Topics {
    fn add(&self, builder: TopicBuilder) -> Result<TopicId>;
    fn list(&self) -> Result<Vec<Topic>>;
    fn remove(&self, id: TopicId) -> Result<()>;
}
