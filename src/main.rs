mod console;
mod model;

use console::enter::enter;

fn main() {
    // TODO: This is silly. Just parse the args directly in this file.
    enter().unwrap()
}
