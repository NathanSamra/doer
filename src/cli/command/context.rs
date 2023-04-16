use crate::database::DATABASE;

pub fn show_context() {
    let database = DATABASE.lock().unwrap();
    println!("{}", database.context())
}

pub fn set_context(context: String) {
    let mut database = DATABASE.lock().unwrap();
    database.set_context(context);
}

pub fn list_contexts() {
    let database = DATABASE.lock().unwrap();
    for context in database.contexts() {
        println!("{}", context)
    }
}
