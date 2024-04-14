use crate::model::day::Day;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

pub type Year = HashMap<NaiveDate, Day>;

#[derive(Serialize)]
pub struct YearDataFile {
    version: String,
    pub days: Year,
}

impl YearDataFile {
    pub fn new(version: String, year: Year) -> Self {
        Self {
            version,
            days: year,
        }
    }
}

impl Default for YearDataFile {
    fn default() -> Self {
        todo!()
    }
}

impl<'de> Deserialize<'de> for YearDataFile {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
