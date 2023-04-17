use crate::model::day::Day;
use chrono::NaiveDate;
use std::collections::HashMap;

pub type Context = HashMap<NaiveDate, Day>;
