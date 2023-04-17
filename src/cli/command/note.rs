use crate::cli::date_parser::today;
use crate::database::DATABASE;
use crate::model::day::Day;

pub fn note(note: String) {
    let mut database = DATABASE.lock().unwrap();
    let date = today();
    let mut day = match database.get(&date) {
        None => Day::default(),
        Some(day_ref) => day_ref.clone(),
    };

    day.note(note);
    database.set(date, day);
}
