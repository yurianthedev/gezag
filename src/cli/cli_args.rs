use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author = "yurian", version, about = "Manage your resources")]
pub struct CliArgs {
    #[command(subcommand)]
    pub subcommand: CliSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommands {
    Config,
    Resources(ResourcesArgs),
}

#[derive(Debug, Args)]
pub struct ResourcesArgs {
    #[command(subcommand)]
    pub action: CliResourcesActions,
}

#[derive(Debug, Subcommand)]
pub enum CliResourcesActions {
    Add(ResourcesAddArgs),
    List(ResourcesListArgs),
}

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("by").required(true).args(["all", "author"])))]
pub struct ResourcesListArgs {
    #[arg(long)]
    pub all: bool,
    #[arg(long)]
    pub author: Option<String>,
}

#[derive(Debug, Args)]
pub struct ResourcesAddArgs {
    #[arg(value_enum)]
    pub kind: CliArgsResourcesKinds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliArgsResourcesKinds {
    Book,
}
