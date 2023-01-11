use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::topic::TopicId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub id: ResourceId,
    pub kind: Kind,
    pub topics: Vec<TopicId>,
}

impl Resource {
    pub fn builder() -> ResourceBuilder {
        ResourceBuilder::default()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceId(pub Uuid);

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceIndeces {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceKeys {}

/// Intended to make easy to build a resource and also as a placeholder to resources before getting an id.
#[derive(Default)]
pub struct ResourceBuilder {
    pub kind: Option<Kind>,
    pub topics: Vec<TopicId>,
}

impl ResourceBuilder {
    pub fn new() -> Self {
        Self {
            kind: None,
            topics: Vec::new(),
        }
    }

    pub fn with_kind(mut self, kind: Kind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn with_topic(mut self, topic: TopicId) -> Self {
        self.topics.push(topic);
        self
    }

    pub fn with_topics(mut self, topics: Vec<TopicId>) -> Self {
        let mut topics = topics;
        self.topics.append(&mut topics);
        self
    }

    pub fn is_ready(&self) -> bool {
        self.kind.is_some()
    }

    /// This will trow if the builder is not ready. Plase check with `Self::is_ready` function.
    pub fn build_with_id(self, id: Uuid) -> Resource {
        Resource {
            id: ResourceId(id),
            kind: self.kind.unwrap(),
            topics: self.topics,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum Kind {
    Book { title: String, authors: Vec<String> },
}

pub struct BookBuilder {
    title: Option<String>,
    authors: Vec<String>,
}

impl BookBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            authors: Vec::new(),
        }
    }

    pub fn with_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn with_author(&mut self, author: String) -> &mut Self {
        self.authors.push(author);
        self
    }

    pub fn with_authors(mut self, authors: Vec<String>) -> Self {
        let mut authors = authors;
        self.authors.append(&mut authors);
        self
    }

    pub fn is_ready(&self) -> bool {
        self.title.is_some()
    }

    /// This will trow if the builder is not ready. Plase check with `Self::is_ready` function.
    pub fn build(self) -> Kind {
        Kind::Book {
            title: self.title.unwrap(),
            authors: self.authors,
        }
    }
}
