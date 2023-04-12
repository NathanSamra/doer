use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Plan priorities
    Plan,
    /// Copy priorities
    Copy,
    /// Show priorities
    Show,
    /// Tick priorities
    Tick,
    /// Work context
    Context,
    /// Current focus
    Focus,
    /// Break from work
    Break,
    /// End the day
    EndDay,
    /// Add a note to the day
    Note,
}
