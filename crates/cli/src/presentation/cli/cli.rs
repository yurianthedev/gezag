use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use directories::UserDirs;
use inquire::{Confirm, Text};
use std::{env, ffi::OsString, path::Path};

use crate::domain::{
    entities::resource::{BookBuilder, ResourceBuilder},
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
                    let mut builder = BookBuilder::new();
                    builder.with_title(Text::new("Title").prompt()?);
                    loop {
                        builder.with_author(Text::new("Author").prompt()?);
                        if !Confirm::new("Do you want to add another one?").prompt()? {
                            break;
                        }
                    }

                    let kind = builder.build();
                    let builder = ResourceBuilder::new();
                    resources_repo.add(builder.with_kind(kind))?;
                }
            },
            Actions::List(l) => {
                if l.all {
                    resources_repo
                        .list()?
                        .iter()
                        .for_each(|el| println!("{:?}", el));
                }
                //     resources_repo.list_by();
                // )?
                // .iter()
                // .for_each(|el| println!("{:?}", el));
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
