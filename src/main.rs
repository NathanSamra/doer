mod config;
mod console;
mod model;
mod storage;

use console::enter::enter;

// TODO: Use the tracing package to create a log file.
// TODO: Catch and handle any errors that get to this point. Use color-eyre to format the message.
fn main() {
    // TODO: This is silly. Just parse the args directly in this file.
    enter().unwrap()
}
