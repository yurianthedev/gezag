use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{topic::TopicId, Indexable, Keyable};

#[derive(Serialize, Deserialize, Debug, Builder)]
pub struct Resource {
    pub id: ResourceId,
    #[serde(flatten)]
    pub kind: Kind,
    #[builder(default)]
    pub topics: Vec<TopicId>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash)]
pub struct ResourceId(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResourceKeys {
    Id(ResourceId),
}

impl Keyable<ResourceKeys> for Resource {
    fn is(&self, key: &ResourceKeys) -> bool {
        match key {
            ResourceKeys::Id(id) => id.eq(&self.id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceIndeces {
    TopicId(TopicId),
}

impl Indexable<ResourceIndeces> for Resource {
    fn is_related_to(&self, index: &ResourceIndeces) -> bool {
        match index {
            ResourceIndeces::TopicId(ti) => self.topics.contains(ti),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", tag = "kind")]
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

    pub fn with_authors(&mut self, authors: Vec<String>) -> &mut Self {
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
