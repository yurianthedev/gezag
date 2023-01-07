pub mod config;

use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use directories::UserDirs;
use std::{collections::HashSet, env, ffi::OsString, path::Path};

use crate::{
    entities::{
        resources::{self, Resource},
        Index,
    },
    indexer::Indexer,
    repositories::{repository, ResourceRepository},
};

/// gezag resource add book.
/// gezag topic add --name "Machine Learning".
/// gezag resource list --topic "Machine Learning".
#[derive(Parser)]
#[command(author = "yurian", version, about = "Manage your resources")]
pub struct Cli {
    #[command(subcommand)]
    pub entity: Entities,
    #[arg(short, long = "config")]
    pub config_location: Option<OsString>,
}

impl Cli {
    pub fn config(&self) {
        self.config_location
            .clone()
            .map(|p| Path::new(&p).to_owned())
            .or_else(|| env::var_os("GEZAG_BASE_DIR").map(|p| Path::new(&p).to_owned()))
            .or_else(|| UserDirs::new().map(|uds| uds.home_dir().to_owned()));
    }

    pub fn run(&self, indexer: impl Indexer) {
        match &self.entity {
            Entities::Resources(res) => Self::resources(res, indexer).unwrap(),
        };
    }

    fn resources(
        resources: &Resources,
        resources_repo: impl ResourceRepository,
    ) -> Result<(), anyhow::Error> {
        match &resources.action {
            Actions::Add(add) => match add.kind {
                Kind::Book => {
                    print!("Title: ");
                    let title: String = text_io::read!("Title: {}\n");
                    repository::Add::add(
                        &resources_repo,
                        Resource {
                            id: uuid::Uuid::new_v4(),
                            kind: resources::Kind::Book {
                                title: "Bruh".to_string(),
                                author: "Bruh momemnto".to_string(),
                            },
                            topics: HashSet::new(),
                        },
                    )?;
                }
            },
            Actions::List(l) => {
                if l.all {
                    repository::List::list(&resources_repo)?
                        .iter()
                        .for_each(|el| println!("{:?}", el));
                } else if l.author.is_some() {
                    repository::ListBy::list_by(
                        &resources_repo,
                        &resources::Indeces::BookIndex(resources::BookIndeces::Author(
                            l.author.clone().unwrap(),
                        )),
                    )?
                    .iter()
                    .for_each(|el| println!("{:?}", el));
                }
            }
        };

        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Entities {
    Resources(Resources),
}

#[derive(Args)]
pub struct Resources {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
    Add(Add),
    List(List),
}

#[derive(Args)]
#[command(group(ArgGroup::new("by").required(true).args(["all", "author"])))]
pub struct List {
    #[arg(long)]
    all: bool,
    #[arg(long)]
    author: Option<String>,
}

#[derive(Args)]
pub struct Add {
    #[arg(value_enum)]
    pub kind: Kind,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Kind {
    Book,
}
