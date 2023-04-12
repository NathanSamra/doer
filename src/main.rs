mod cli;

use crate::cli::{Cli, Command};

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Plan => {}
        Command::Copy => {}
        Command::Show => {}
        Command::Tick => {}
        Command::Context => {}
        Command::Focus => {}
        Command::Break => {}
        Command::EndDay => {}
        Command::Note => {}
    }
}
