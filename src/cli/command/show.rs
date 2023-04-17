use crate::database::DATABASE;
use chrono::NaiveDate;

pub fn show(date: &NaiveDate) {
    let database = DATABASE.lock().unwrap();
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

pub fn show_last() {
    let database = DATABASE.lock().unwrap();
    match database.most_recent_past() {
        None => {
            println!("No entries to show")
        }
        Some(date) => show(date),
    }
}
