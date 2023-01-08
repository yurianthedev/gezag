pub mod config;

use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use directories::UserDirs;
use std::{collections::HashSet, env, ffi::OsString, path::Path};
use text_io::read;

use crate::domain::{
    entities::resource::ResourceBuilder,
    repositories::{Librarian, ResourcesRepository},
};

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
            .or_else(|| UserDirs::new().map(|uds| uds.home_dir().to_owned()))
            .unwrap();
    }

    pub fn run(&self, indexer: impl Librarian) {
        match &self.entity {
            Entities::Resources(res) => Self::resources(res, indexer).unwrap(),
        };
    }

    fn resources(
        resources: &Resources,
        resources_repo: impl ResourcesRepository,
    ) -> Result<(), anyhow::Error> {
        match &resources.action {
            Actions::Add(add) => match add.kind {
                Kind::Book => {
                    print!("Title: ");
                    let title: String = read!("{}\n");
                    print!("Author: ");
                    let author: String = read!("{}\n");

                    resources_repo.add(ResourceBuilder {});
                }
            },
            Actions::List(l) => {
                if l.all {
                    resources_repo
                        .list()?
                        .iter()
                        .for_each(|el| println!("{:?}", el));
                } else if l.author.is_some() {
                    todo!()
                    //     resources_repo.list_by();
                    // )?
                    // .iter()
                    // .for_each(|el| println!("{:?}", el));
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
