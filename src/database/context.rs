use crate::model::day::Day;
use chrono::NaiveDate;
use std::collections::BTreeMap;

pub type Context = BTreeMap<NaiveDate, Day>;
