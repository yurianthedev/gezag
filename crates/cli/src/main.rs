use std::{collections::HashSet, path::Path};

use base::{
    domain::{repository::Add, resource::Book},
    infrastructure::shelf::local::Local,
};

fn main() {
    let mut local = Local::new::<Book>(Path::new("./index.toml")).unwrap();
    let book = Book {
        title: "Hi".to_string(),
        authors: HashSet::new(),
        first_published_at: None,
        revisions: Vec::new(),
    };

    local.add(book).unwrap();
}
