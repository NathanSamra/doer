mod client;
mod command;
mod date_parser;

pub use client::Error;

use crate::cli::command::Command;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn run(&self) -> Result<(), client::Error> {
        self.command.execute()
    }
}
