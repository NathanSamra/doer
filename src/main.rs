#![feature(once_cell)]

mod cli;
mod database;
mod model;
mod today;

use crate::cli::Cli;

use clap::Parser;

fn main() -> Result<(), cli::Error> {
    let cli = Cli::parse();
    cli.run()
}
