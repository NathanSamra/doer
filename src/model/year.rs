use crate::model::day::Day;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Year = HashMap<NaiveDate, Day>;

// TODO: Create a separate .exe for upgrading version schemas. Something like migrate_doer_data.exe?
// TODO: As a middle point should also create a generate_doer_schema.exe.
#[derive(Deserialize, Serialize)]
pub struct YearDataFile {
    version: String,
    pub days: Year,
}

impl YearDataFile {
    pub fn new(year: Year) -> Self {
        Self {
            version: VERSION.to_string(),
            days: year,
        }
    }
}

impl Default for YearDataFile {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            days: Year::new(),
        }
    }
}
