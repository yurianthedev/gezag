use clap::Parser;

use std::fs::{self};

use super::{
    config::{CliConfig, CliConfigProvider, CliLibrarians, LocalRegistryConfig},
    prompts, CliArgs, CliArgsResourcesActions, CliArgsResourcesKinds, CliArgsSubcommands,
};
use crate::{
    data::repositories::local,
    domain::entities::resource::ResourceBuilder,
    domain::repositories::{Librarian, Resources},
};

pub struct Cli {
    args: CliArgs,
    config_provider: CliConfigProvider,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            args: CliArgs::parse(),
            config_provider: Default::default(),
        }
    }
}

impl Cli {
    /// As this is the executer, is the one responsible for the error reporting; that's
    /// why it just panic instead of returning a result further.
    pub fn run(&self) {
        match &self.args.subcommand {
            CliArgsSubcommands::Config => self.config().unwrap(),
            CliArgsSubcommands::Resources(_) => {
                let librarian = self
                    .create_libarian()
                    .expect("Error while creating a librarian. We suggest to run `config`.");
                self.process(librarian).unwrap();
            }
        };
    }

    fn create_libarian(&self) -> Result<impl Resources, anyhow::Error> {
        let config = self.config_provider.read()?;
        match &config.librarian {
            CliLibrarians::Local { registry } => Ok(local::Librarian::new(&registry.location)?),
        }
    }

    /// We asume `Config` is unreachable at this point because it does not need a librarian to run.
    fn process(&self, librarian: impl Librarian) -> Result<(), anyhow::Error> {
        match &self.args.subcommand {
            CliArgsSubcommands::Resources(rsrc_args) => match &rsrc_args.action {
                CliArgsResourcesActions::Add(add_args) => match &add_args.kind {
                    CliArgsResourcesKinds::Book => {
                        let book_builder = prompts::add_book()?;
                        let rsrc_builder = ResourceBuilder::default()
                            .kind(book_builder.build())
                            .to_owned();
                        librarian.add(rsrc_builder)?;
                        Ok(())
                    }
                },
                CliArgsResourcesActions::List(list_args) => {
                    if list_args.all {
                        librarian.list()?.iter().for_each(|el| println!("{el:?}"));
                    }

                    Ok(())
                }
            },
            CliArgsSubcommands::Config => unreachable!(),
        }
    }

    fn config(&self) -> Result<(), anyhow::Error> {
        let home_path = self.config_provider.app_home_path();
        let config_file_path = self.config_provider.config_file_path();
        let mut cli_config: Option<CliConfig> = None; // We'd like to check if there's already a configuration we can use to prompt it as default.

        if home_path.exists() && config_file_path.exists() && config_file_path.is_file() {
            cli_config = Some(self.config_provider.read()?);
        } else {
            fs::create_dir_all(&home_path)?;
        }
        // At this point the app's home dir should be waranteed to exist.

        let librarians = vec!["Local".to_string()]; // TODO: I don't know where to look up for this. I can imagine it can come from a list of "plugins" the user has installed on the cli, but for now that functionality does not even exists.
        let chosen = prompts::choose_librarian(librarians)?;

        // TODO: I'd be nice if this match could be made against an enum.
        match chosen.as_ref() {
            "Local" => {
                let current = cli_config.and_then(|user_config| match user_config.librarian {
                    CliLibrarians::Local { registry } => Some(registry.location),
                });
                let rg_location = prompts::registry_location(current)?;
                let config = CliConfig {
                    librarian: CliLibrarians::Local {
                        registry: LocalRegistryConfig {
                            location: rg_location,
                        },
                    },
                };
                self.config_provider.write(config)?;
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}
