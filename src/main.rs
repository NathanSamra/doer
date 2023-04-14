#![feature(once_cell)]

mod cli;
mod config;
mod model;

use crate::cli::Cli;

use clap::Parser;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
