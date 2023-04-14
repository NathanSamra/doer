use crate::database::DATABASE;

pub fn show_context() {
    println!("{}", DATABASE.context())
}

pub fn set_context(_context: String) {
    todo!()
}

pub fn list_contexts() {
    todo!()
}
