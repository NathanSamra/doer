use crate::database::Database;

pub fn show_context(database: &Database) {
    println!("{}", database.context())
}

pub fn set_context(database: &mut Database, context: String) {
    database.set_context(context);
}

pub fn list_contexts(database: &Database) {
    for context in database.contexts() {
        println!("{}", context)
    }
}
