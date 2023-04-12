mod cli;

use crate::cli::Cli;

use clap::Parser;

fn main() {
    Cli::parse();
}
