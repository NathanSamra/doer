mod cli;
mod database;
mod metadata;
mod model;
mod storage;

use crate::cli::cli_parser::{CliParser, Command};
use crate::cli::controller::{Controller, PriorityId};
use crate::storage::storage_handler::{default_dirs, StorageHandler};
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
    let mut controller = Controller::new(StorageHandler::connect(default_dirs()).unwrap());
    match args.command {
        Command::AddTask { date, task } => controller.add_task(date_from_arg(date.as_str())?, task),
        Command::Plan { date } => controller.plan_priorities(date_from_arg(date.as_str())?),
        Command::CopyPriorities { from, to } => {
            controller.copy_priorities(&date_from_arg(from.as_str())?, &date_from_arg(to.as_str())?)
        }
        Command::Show { date } => controller.show(&date_from_arg(date.as_str())?),
        Command::ShowLast {} => {
            controller.show_last();
        }
        Command::Tick { id } => controller.tick(id - 1),
        Command::UnTick { id } => controller.un_tick(id - 1),
        Command::Context {} => controller.context(),
        Command::ListContexts {} => controller.contexts(),
        Command::NewContext { context } => controller.new_context(context),
        Command::SetContext { context } => controller.set_context(context),
        Command::StartFocus { focus } => match focus.parse::<PriorityId>() {
            Ok(id) => controller.start_focus(id - 1),
            Err(_) => controller.start_focus_on_new_task(focus),
        },
        Command::StartBreak {} => controller.start_break(),
        Command::StartDay {} => controller.start_day(),
        Command::EndDay {} => controller.end_day(),
        Command::Note { note } => controller.add_note(note),
        Command::RemoveLock {} => controller.remove_lock(),
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
