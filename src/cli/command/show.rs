use crate::database::Database;
use chrono::NaiveDate;

pub fn show(database: &Database, date: &NaiveDate) {
    match database.get(date) {
        None => {
            println!("No entry for {}", date)
        }
        Some(day) => {
            println!("{}", date);
            println!("{}", day);
        }
    }
}

pub fn show_last(database: &Database) {
    match database.most_recent_past() {
        None => {
            println!("No entries to show")
        }
        Some(date) => show(database, date),
    }
}
