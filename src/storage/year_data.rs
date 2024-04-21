use crate::database::Database;
use crate::metadata::app_version;
use crate::model::day::Day;
use chrono::Datelike;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: Create a separate .exe for upgrading version schemas. Something like migrate_doer_data.exe?
// TODO: As a middle point should also create a generate_doer_schema.exe.
/// A struct containing all the data to recreate a year of Doer. Each year contains all the tasks
/// referenced in its days. This means some tasks are duplicated over different year files, and in
/// this case the latest Task will always be parsed as the correct one.
#[derive(Deserialize, Serialize)]
pub struct YearData {
    version: String,
    year: i32,
    days: HashMap<NaiveDate, Day>,
}

impl YearData {
    pub fn new(year: i32) -> Self {
        Self {
            version: app_version().to_string(),
            year,
            days: HashMap::new(),
        }
    }

    pub fn year(&self) -> &i32 {
        &self.year
    }
}

impl From<Database> for Vec<YearData> {
    fn from(database: Database) -> Self {
        let mut years: HashMap<i32, YearData> = HashMap::new();

        for (date, day) in database.days() {
            let year = date.year();
            let year_data = years.entry(year).or_insert_with(|| YearData::new(year));
            year_data.days.insert(*date, day.clone());
        }

        years.into_values().collect()
    }
}

impl From<YearData> for Database {
    fn from(_value: YearData) -> Self {
        todo!()
    }
}
