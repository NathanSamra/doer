mod context;
mod copy;
mod focus;
mod note;
mod plan;
mod show;
mod tick;

use crate::cli::command::context::{list_contexts, set_context, show_context};
use crate::cli::command::copy::copy;
use crate::cli::command::focus::{end_break, end_day, set_focus, show_focus, start_break};
use crate::cli::command::note::note;
use crate::cli::command::plan::plan;
use crate::cli::command::show::{show, show_last};
use crate::cli::command::tick::set_tick;

use crate::cli::date_parser::parse_date;

use crate::model::focus::Focus;
use crate::model::priority::PriorityId;

use crate::today::today;
use chrono::NaiveDate;
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
            Command::Plan(args) => plan(&args.date),
            Command::Copy(args) => copy(&args.from, &args.to),
            Command::Show(args) => show(&args.date),
            Command::ShowLast => show_last(),
            Command::Tick(args) => {
                let date = args.date.unwrap_or_else(today);
                set_tick(date, args.priority_id, !args.reset);
            }
            Command::Context(args) => match &args.command {
                ContextCommand::Show => show_context(),
                ContextCommand::List => list_contexts(),
                ContextCommand::Set { context } => set_context(context.clone()),
            },
            Command::Focus(args) => match &args.command {
                FocusCommand::Show => show_focus(),
                FocusCommand::Set { focus } => set_focus(focus),
                FocusCommand::StartBreak => start_break(),
                FocusCommand::EndBreak => end_break(),
                FocusCommand::EndDay => end_day(),
            },
            Command::Note(args) => note(args.note.clone()),
        }
    }
}

#[derive(Args)]
pub struct PlanArgs {
    /// Date to plan
    #[arg(value_parser = parse_date, default_value_t = today())]
    date: NaiveDate,
}

#[derive(Args)]
pub struct CopyArgs {
    #[arg(value_parser = parse_date)]
    /// Date to copy from
    from: NaiveDate,
    #[arg(value_parser = parse_date)]
    /// Date to copy to
    to: NaiveDate,
}

#[derive(Args)]
pub struct ShowArgs {
    /// Date to plan
    #[arg(value_parser = parse_date, default_value_t = today())]
    date: NaiveDate,
}

#[derive(Args)]
pub struct TickArgs {
    /// Priority to tick
    priority_id: PriorityId,
    /// Date, otherwise today
    #[arg(short, long, value_parser = parse_date)]
    date: Option<NaiveDate>,
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
