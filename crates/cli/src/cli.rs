
use clap::{Args, Parser, Subcommand, ValueEnum};

/// gezag resource add book.
/// gezag topic add --name "Machine Learning".
/// gezag resource list --topic "Machine Learning".
#[derive(Parser)]
#[command(author = "yurian", version, about = "Manage your resources")]
pub struct Cli {
    #[command(subcommand)]
    pub entity: Entities,
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

pub mod config {
    use std::path::Path;

    pub struct CoupledConfig<'a> {
        config_location: &'a Path,
    }
}
