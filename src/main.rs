mod cli;
mod data;
pub mod edit_day_guard;
mod metadata;
mod model;
mod storage;

use crate::cli::cli_parser::{CliParser, Command};
use crate::cli::commands::{
    add_note, context, contexts, copy_priorities, end_break, end_day, plan_priorities, set_context,
    set_focus, set_focus_to_priority, show, show_last, start_break, tick, un_tick, PriorityId,
};
use chrono::{Days, Local, NaiveDate, ParseResult, Weekday};
use clap::Parser;

// TODO: All unwraps should at least be expects() with a message.
// TODO: Use the tracing package to create a log file.
// TODO: Catch and handle any errors that get to this point. Maybe Use color-eyre to format the message better instead of anyhow.
fn main() -> anyhow::Result<()> {
    let args = CliParser::parse();
    execute_command(args)?;
    Ok(())
}

// TODO: Print error messages in a nice way instead of getting the stack trace. The trace should
// just be for very rare cases.
fn execute_command(args: CliParser) -> ParseResult<()> {
    match args.command {
        Command::Plan { date } => plan_priorities(date_from_arg(date.as_str())?),
        Command::CopyPriorities { from, to } => {
            copy_priorities(&date_from_arg(from.as_str())?, &date_from_arg(to.as_str())?)
        }
        Command::Show { date } => show(&date_from_arg(date.as_str())?),
        Command::ShowLast {} => {
            show_last();
        }
        Command::Tick { id } => tick(id - 1),
        Command::UnTick { id } => un_tick(id - 1),
        Command::Context {} => context(),
        Command::Contexts {} => contexts(),
        Command::SetContext { context } => set_context(context),
        Command::SetFocus { focus } => match focus.parse::<PriorityId>() {
            Ok(id) => set_focus_to_priority(id - 1),
            Err(_) => set_focus(focus.as_str()),
        },
        Command::StartBreak {} => start_break(),
        Command::EndBreak {} => end_break(),
        Command::EndDay {} => end_day(),
        Command::Note { note } => add_note(note),
    }

    Ok(())
}

fn date_from_arg(arg: &str) -> ParseResult<NaiveDate> {
    let today = Local::now().date_naive();
    let day = Days::new(1);
    let first_week_day = today.week(Weekday::Mon).first_day();

    match arg {
        "yesterday" => Ok(today - day),
        "today" => Ok(today),
        "tomorrow" => Ok(today + day),
        "monday" => Ok(first_week_day),
        "tuesday" => Ok(first_week_day + day),
        "wednesday" => Ok(first_week_day + day + day),
        "thursday" => Ok(first_week_day + day + day + day),
        "friday" => Ok(first_week_day + day + day + day + day),
        "saturday" => Ok(first_week_day + day + day + day + day + day),
        "sunday" => Ok(first_week_day + day + day + day + day + day + day),
        _ => NaiveDate::parse_from_str(arg, "%Y-%m-%d"),
    }
}
