use crate::cli::smart_date::SmartDate;
use crate::priority::PriorityId;

use crate::focus::Focus;
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
    Tick(TickArgs),
    /// Work context
    Context(ContextArgs),
    /// Current focus
    Focus(FocusArgs),
    /// Add a note to the day
    Note(NoteArgs),
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
            Command::Tick(_tick_args) => {
                todo!()
            }
            Command::Context(_context_args) => {
                todo!()
            }
            Command::Focus(_focus_args) => {
                todo!()
            }
            Command::Note(_note_args) => {
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
    /// Date, otherwise today
    #[arg(short, long)]
    date: Option<SmartDate>,
    /// Reset tick
    #[arg(short, long)]
    reset: bool,
}

#[derive(Args)]
pub struct ContextArgs {
    /// Context command
    #[command(subcommand)]
    command: ContextCommand,
}

#[derive(Subcommand)]
pub enum ContextCommand {
    /// Show current context
    Show,
    /// List all contexts
    List,
    /// Set context
    Set { context: String },
}

#[derive(Args)]
pub struct FocusArgs {
    /// Focus command
    #[command(subcommand)]
    command: FocusCommand,
}

#[derive(Subcommand)]
pub enum FocusCommand {
    /// Show current context
    Show,
    /// Set context
    Set { focus: Focus },
    /// Start focus break
    StartBreak,
    /// End focus break
    EndBreak,
    /// End focus
    EndDay,
}

#[derive(Args)]
pub struct NoteArgs {
    /// Note
    note: String,
}
