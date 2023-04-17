use chrono::{Local, NaiveDate};

pub fn today() -> NaiveDate {
    Local::now().naive_local().date()
}
