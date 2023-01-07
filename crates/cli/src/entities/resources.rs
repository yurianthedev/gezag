use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use super::{topics::Topic, Index, Key};

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub id: Uuid,
    pub kind: Kind,
    pub topics: HashSet<Topic>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum Kind {
    Book { title: String, author: String },
}

pub enum Keys {
    Id(Uuid),
    AuthorTitle { title: String, author: String },
}

impl Key<Resource> for Keys {
    fn is(&self, el: &Resource) -> bool {
        match self {
            Keys::Id(id) => el.id.eq(id),
            Keys::AuthorTitle { title, author } => match &el.kind {
                Kind::Book {
                    title: t,
                    author: a,
                } => title.eq(t) && author.eq(a),
            },
        }
    }
}

pub enum Indeces {
    Topic(Topic),
    BookIndex(BookIndeces),
}

impl Index<Resource> for Indeces {
    fn is_related(&self, el: &Resource) -> bool {
        match self {
            Self::Topic(topic) => el.topics.contains(topic),
            Self::BookIndex(bi) => match &el.kind {
                Kind::Book { title: _, author } => match bi {
                    BookIndeces::Author(a) => author == a,
                },
            },
        }
    }
}

pub enum BookIndeces {
    Author(String),
}
