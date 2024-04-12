use crate::console::client::{Client, PriorityId};
use chrono::NaiveDate;
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

fn date_from_arg(_arg: &str) -> NaiveDate {
    todo!()
}

pub fn enter() {
    let args = Cli::parse();
    match &args.command {
        Command::Plan { date } => {
            let mut client = Client::new();
            client.plan_priorities(&date_from_arg(date))
        }
        Command::Copy { from, to } => {
            let mut client = Client::new();
            client.copy_priorities(&date_from_arg(from), &date_from_arg(to))
        }
        Command::Show { date } => {
            let client = Client::new();
            client.show(&date_from_arg(date))
        }
        Command::ShowLast {} => {
            let client = Client::new();
            let date = client.last_date();
            client.show(&date)
        }
        Command::Tick { id } => {
            let mut client = Client::new();
            client.tick(&(id - 1))
        }
        Command::UnTick { id } => {
            let mut client = Client::new();
            client.un_tick(&(id - 1))
        }
        Command::Context {} => {
            let client = Client::new();
            client.context()
        }
        Command::Contexts {} => {
            let client = Client::new();
            client.contexts()
        }
        Command::SetContext { context } => {
            let mut client = Client::new();
            client.set_context(context)
        }
        Command::SetFocus { focus } => {
            let mut client = Client::new();
            match focus.parse::<PriorityId>() {
                Ok(id) => client.set_focus_to_priority(id - 1),
                Err(_) => client.set_focus(focus),
            }
        }
        Command::StartBreak {} => {
            let mut client = Client::new();
            client.start_break()
        }
        Command::EndBreak {} => {
            let mut client = Client::new();
            client.end_break()
        }
        Command::EndDay {} => {
            let mut client = Client::new();
            client.end_day()
        }
        Command::Note { note } => {
            let mut client = Client::new();
            client.note(note)
        }
    }
}
