pub mod data;
pub mod domain;
pub mod presentation;

use presentation::cli::Cli;

fn main() {
    let cli = Cli::default();
    cli.run();
}
