mod client;
mod command;
mod date_parser;

use crate::cli::command::Command;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn run(&self) {
        self.command.execute()
    }
}
