use crate::cli::commands::PriorityId;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliParser {
    #[command(subcommand)]
    pub command: Command,
}

// TODO: Add descriptions for the commands.
// TODO: Rethink all of these. Maybe come up with a resource based approach instead of actions
#[derive(Subcommand, Debug)]
pub enum Command {
    AddTask {
        date: String,
        task: String,
    },
    Plan {
        /// Date to plan
        date: String,
    },
    // TODO: I think instead there should also be methods for moving tasks along or copying everything
    CopyPriorities {
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
    StartFocus {
        // TODO: Consider being more explicit over ID or name, i.e two different variables here.
        /// name or ID of focus
        focus: String,
    },
    StartBreak {},
    StartDay {},
    EndDay {},
    Note {
        /// Note for today
        note: String,
    },
}
