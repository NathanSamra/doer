use crate::cli::controller::PriorityId;
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
    NewTask {
        #[arg(short, long, default_value = "today")]
        date: String,
        task: String,
    },
    Plan {
        /// Date to plan
        #[arg(short, long, default_value = "today")]
        date: String,
    },
    // TODO: I think instead there should also be methods for moving tasks along or copying everything
    CopyTasks {
        /// Date/day to copy from
        from: String,
        /// Date/day to copy to
        to: String,
        #[arg(short, long, default_value_t = False)]
        include_unfinished: bool,
    },
    Show {
        /// Date to show
        #[arg(short, long, default_value = "today")]
        date: String,
    },
    ShowLast {},
    Tick {
        // TODO: Add description
        // TODO: It's not a priority ID is it
        id: PriorityId,
        #[arg(short, long, default_value = "today")]
        date: String,
    },
    UnTick {
        // TODO: Add description
        id: PriorityId,
        #[arg(short, long, default_value = "today")]
        date: String,
    },
    Context {},
    ListContexts {},
    SetContext {
        /// Context to switch to
        context: String,
    },
    NewContext {
        /// Context to create
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
        #[arg(short, long, default_value = "today")]
        date: String,
    },
    RemoveLock {},
}
