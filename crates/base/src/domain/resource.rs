use std::collections::HashSet;

use chrono::naive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Book {
    pub title: String,
    pub authors: HashSet<String>,
    pub first_published_at: Option<naive::NaiveDate>,
    pub revisions: Vec<Revision>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Revision {
    edition: String,
    published_at: Option<naive::NaiveDate>,
    publisher: Option<String>,
}
