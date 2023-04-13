use crate::priority::PriorityId;
use chrono::{Local, NaiveDate};
use clap::{Args, Parser, Subcommand};

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

#[derive(Subcommand)]
enum Command {
    /// Plan priorities
    Plan(PlanArgs),
    /// Copy priorities
    Copy(CopyArgs),
    /// Show priorities
    Show(ShowArgs),
    /// Show last given priorities
    ShowLast,
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

impl Command {
    pub fn execute(&self) {
        match &self {
            Command::Plan(_plan_args) => {
                todo!()
            }
            Command::Copy(_copy_args) => {
                todo!()
            }
            Command::Show(_show_args) => {
                todo!()
            }
            Command::ShowLast => {
                todo!()
            }
            Command::Tick => {
                todo!()
            }
            Command::Context => {
                todo!()
            }
            Command::Focus => {
                todo!()
            }
            Command::Break => {
                todo!()
            }
            Command::EndDay => {
                todo!()
            }
            Command::Note => {
                todo!()
            }
        }
    }
}

#[derive(Args)]
struct PlanArgs {
    /// Date to plan
    #[arg(default_value_t = today())]
    date: NaiveDate,
}

#[derive(Args)]
struct CopyArgs {
    /// Date to copy from
    from: NaiveDate,
    /// Date to copy to
    to: NaiveDate,
}

#[derive(Args)]
struct ShowArgs {
    /// Date to plan
    #[arg(default_value_t = today())]
    date: NaiveDate,
}

#[derive(Args)]
struct TickArgs {
    /// Priority to tick
    priority_id: PriorityId,
    /// Date
    #[arg(default_value_t = today())]
    date: NaiveDate,
    /// Reset tick
    reset: bool,
}

fn today() -> NaiveDate {
    Local::now().naive_local().date()
}
