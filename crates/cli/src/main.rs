mod cli;
mod entities;
mod indexer;
mod repositories;

use clap::Parser;

use crate::cli::*;
use crate::indexer::local;

fn main() {
    let cli = Cli::parse();
    let local_indexer = local::Indexer::new("index.json").unwrap();
    cli.run(local_indexer);
}
