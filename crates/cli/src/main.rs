pub mod data;
pub mod domain;
pub mod presentation;

use clap::Parser;

use crate::data::repositories::local;
use crate::presentation::cli::Cli;

fn main() {
    let cli = Cli::parse();
    let local_librarian = local::Librarian::new("index.json").unwrap();
    cli.run(local_librarian);
}
