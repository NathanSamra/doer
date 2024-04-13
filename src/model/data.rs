use crate::model::day::Day;
use chrono::NaiveDate;
use std::fs;
use std::path::PathBuf;

pub struct Data {
    root: PathBuf,
    context: String,
}

impl Data {
    pub fn new(database: PathBuf, context: String) -> Self {
        if !database.exists() {
            // TODO: May have to handle this error correctly if it becomes a problem
            fs::create_dir_all(&database).unwrap();
        }

        Self {
            root: database,
            context,
        }
    }

    pub fn day(&self, _date: &NaiveDate) -> Day {
        todo!()
    }

    pub fn set_day(&mut self, _date: &NaiveDate, _day: &Day) {
        todo!()
    }
}
