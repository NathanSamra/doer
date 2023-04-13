use chrono::{NaiveDate, Weekday};
use std::error::Error;

use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum SmartDate {
    Yesterday,
    Today,
    Tomorrow,
    Weekday(Weekday),
    Iso(String),
}

impl Display for SmartDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SmartDate::Yesterday => {
                write!(f, "yesterday")
            }
            SmartDate::Today => {
                write!(f, "today")
            }
            SmartDate::Tomorrow => {
                write!(f, "tomorrow")
            }
            SmartDate::Weekday(weekday) => {
                write!(f, "{}", weekday)
            }
            SmartDate::Iso(iso_str) => {
                write!(f, "{}", iso_str)
            }
        }
    }
}

impl<'s> From<&'s str> for SmartDate {
    fn from(value: &'s str) -> Self {
        match value {
            "yesterday" => SmartDate::Yesterday,
            "today" => SmartDate::Today,
            "tomorrow" => SmartDate::Tomorrow,
            "monday" => SmartDate::Weekday(Weekday::Mon),
            "tuesday" => SmartDate::Weekday(Weekday::Tue),
            "wednesday" => SmartDate::Weekday(Weekday::Wed),
            "thursday" => SmartDate::Weekday(Weekday::Thu),
            "friday" => SmartDate::Weekday(Weekday::Fri),
            "saturday" => SmartDate::Weekday(Weekday::Sat),
            "sunday" => SmartDate::Weekday(Weekday::Sun),
            _ => SmartDate::Iso(value.to_string()),
        }
    }
}

impl From<&SmartDate> for NaiveDate {
    fn from(_value: &SmartDate) -> Self {
        todo!()
    }
}

#[derive(Debug)]
struct SmartDateParseError;

impl Display for SmartDateParseError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for SmartDateParseError {}
