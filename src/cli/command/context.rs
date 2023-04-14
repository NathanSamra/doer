use crate::config::CONFIG;

pub fn show_context() {
    let config = &*CONFIG;
    println!("{}", config.database.context)
}

pub fn set_context(_context: &str) {
    todo!()
}

pub fn list_contexts() {
    todo!()
}
