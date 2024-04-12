use clap::{Parser, Subcommand};

type PriorityId = i32;

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

pub fn enter() {
    let _args = Cli::parse();
}
