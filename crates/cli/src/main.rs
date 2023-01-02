mod cli;
mod entities;
mod index;

use clap::Parser;
use std::collections::HashSet;

use crate::cli::*;
use crate::entities::resources::{self, Add, Resource};
use crate::index::local;

fn main() {
    let command = Cli::parse();
    match command.entity {
        Entities::Resources(res) => match res.action {
            Actions::Add(add) => match add.kind {
                Kind::Book => local::Indexer::new("config.json")
                    .unwrap()
                    .add(Resource {
                        id: uuid::Uuid::new_v4(),
                        metadata: resources::Kind::Book {
                            title: "Bruh".to_string(),
                            author: "Bruh momemnto".to_string(),
                        },
                        topics: HashSet::new(),
                    })
                    .unwrap(),
            },
        },
    };
}
