use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

// TODO: Rethink all of these. Maybe come up with a resource based approach instead of actions
#[derive(Subcommand, Debug)]
enum Command {
    Plan {},
    Copy {},
    Show {},
    ShowLast {},
    Tick {},
    UnTick {},
    Context {},
    Contexts {},
    SetContext {},
    SetFocus {},
    StartBreak {},
    EndBreak {},
    EndDay {},
    Note {},
}

pub fn enter() {
    let _args = Cli::parse();
}
