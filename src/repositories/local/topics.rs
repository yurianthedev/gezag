use anyhow::Result;
use uuid::Uuid;

use crate::{
    entities::topic::{Topic, TopicBuilder, TopicId},
    repositories::Topics,
};

impl Topics for super::Librarian {
    fn add(&self, builder: TopicBuilder) -> Result<TopicId> {
        let mut builder = builder;
        let id = TopicId(Uuid::new_v4());
        builder.id(id.clone());
        let topic = builder.build()?;
        self.update(|registry| registry.topics.push(topic))?;

        Ok(id)
    }

    fn list(&self) -> Result<Vec<Topic>> {
        let topics = self.read()?.topics;
        Ok(topics)
    }

    fn remove(&self, id: TopicId) -> Result<()> {
        self.update(|registry| registry.topics.retain(|topic| topic.id == id))?;
        Ok(())
    }
}
