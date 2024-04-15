use crate::console::commands::{
    add_note, context, contexts, copy_priorities, end_break, end_day, plan_priorities, set_context,
    set_focus, set_focus_to_priority, show, show_last, start_break, tick, un_tick, PriorityId,
};
use chrono::{Days, Local, NaiveDate, ParseError, ParseResult, Weekday};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

// TODO: Add descriptions for the commands.
// TODO: Rethink all of these. Maybe come up with a resource based approach instead of actions
#[derive(Subcommand, Debug)]
enum Command {
    Plan {
        /// Date to plan
        date: String,
    },
    Copy {
        /// Date/day to copy from
        from: String,
        /// Date/day to copy to
        to: String,
    },
    Show {
        /// Date to show
        date: String,
    },
    ShowLast {},
    Tick {
        // TODO: Add description
        id: PriorityId,
    },
    UnTick {
        // TODO: Add description
        id: PriorityId,
    },
    Context {},
    Contexts {},
    SetContext {
        /// Context to switch to
        context: String,
    },
    SetFocus {
        // TODO: Consider being more explicit over ID or name, i.e two different variables here.
        /// name or ID of focus
        focus: String,
    },
    StartBreak {},
    EndBreak {},
    EndDay {},
    Note {
        /// Note for today
        note: String,
    },
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

pub fn enter() -> Result<(), ParseError> {
    let args = Cli::parse();
    match args.command {
        Command::Plan { date } => plan_priorities(date_from_arg(date.as_str())?),
        Command::Copy { from, to } => {
            copy_priorities(&date_from_arg(from.as_str())?, &date_from_arg(to.as_str())?)
        }
        Command::Show { date } => show(&date_from_arg(date.as_str())?),
        Command::ShowLast {} => {
            show_last();
        }
        Command::Tick { id } => tick(&(id - 1)),
        Command::UnTick { id } => un_tick(&(id - 1)),
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
