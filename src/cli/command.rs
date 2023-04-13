use crate::cli::smart_date::SmartDate;
use crate::priority::PriorityId;

use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Command {
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
pub struct PlanArgs {
    /// Date to plan
    #[arg(default_value_t = SmartDate::Today)]
    date: SmartDate,
}

#[derive(Args)]
pub struct CopyArgs {
    /// Date to copy from
    from: SmartDate,
    /// Date to copy to
    to: SmartDate,
}

#[derive(Args)]
pub struct ShowArgs {
    /// Date to plan
    #[arg(default_value_t = SmartDate::Today)]
    date: SmartDate,
}

#[derive(Args)]
pub struct TickArgs {
    /// Priority to tick
    priority_id: PriorityId,
    /// Date
    #[arg(default_value_t = SmartDate::Today)]
    date: SmartDate,
    /// Reset tick
    reset: bool,
}
