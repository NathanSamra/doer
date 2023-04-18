use crate::cli::date_parser::parse_date;

use crate::model::day::PriorityId;

use crate::cli::client;
use crate::cli::client::Client;
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
    pub fn execute(&self) -> Result<(), client::Error> {
        let mut client = Client::default();

        match &self {
            Command::Plan(args) => client.plan(&args.date),
            Command::Copy(args) => client.copy(&args.from, args.to)?,
            Command::Show(args) => client.show(&args.date),
            Command::ShowLast => client.show_last(),
            Command::Tick(args) => {
                let date = args.date.unwrap_or_else(today);
                client.set_tick(date, args.priority_id, !args.reset);
            }
            Command::Context(args) => match &args.command {
                ContextCommand::Show => client.show_context(),
                ContextCommand::List => client.list_contexts(),
                ContextCommand::Set { context } => client.set_context(context.clone()),
            },
            Command::Focus(args) => match &args.command {
                FocusCommand::Show => client.show_focus(),
                FocusCommand::Set { focus } => client.set_focus(focus.clone())?,
                FocusCommand::StartBreak => client.start_break(),
                FocusCommand::EndBreak => client.end_break(),
                FocusCommand::EndDay => client.end_day(),
            },
            Command::Note(args) => client.note(args.note.clone()),
        }

        Ok(())
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
    Set { focus: String },
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
