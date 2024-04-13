mod config;
mod console;
mod model;
mod storage;

use console::enter::enter;

fn main() {
    // TODO: This is silly. Just parse the args directly in this file.
    enter().unwrap()
}
