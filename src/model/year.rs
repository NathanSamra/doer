use crate::model::day::Day;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Year = HashMap<NaiveDate, Day>;

#[derive(Serialize)]
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

impl<'de> Deserialize<'de> for YearDataFile {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
