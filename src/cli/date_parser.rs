use crate::today::today;
use chrono::{Datelike, Days, Local, NaiveDate, ParseResult, Weekday};

fn date_from_weekday(weekday: Weekday) -> NaiveDate {
    let week = Local::now().iso_week();
    NaiveDate::from_isoywd_opt(week.year(), week.week0(), weekday)
        .expect("Week date construction failed")
}

pub fn parse_date(date_str: &str) -> ParseResult<NaiveDate> {
    match date_str {
        "yesterday" => Ok(today() - Days::new(1)),
        "today" => Ok(today()),
        "tomorrow" => Ok(today() + Days::new(1)),
        "monday" => Ok(date_from_weekday(Weekday::Mon)),
        "tuesday" => Ok(date_from_weekday(Weekday::Tue)),
        "wednesday" => Ok(date_from_weekday(Weekday::Wed)),
        "thursday" => Ok(date_from_weekday(Weekday::Thu)),
        "friday" => Ok(date_from_weekday(Weekday::Fri)),
        "saturday" => Ok(date_from_weekday(Weekday::Sat)),
        "sunday" => Ok(date_from_weekday(Weekday::Sun)),
        _ => NaiveDate::parse_from_str(date_str, "%Y-%m-%d"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iso_date() {
        let expected = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();

        let parsed = parse_date("2022-07-15").unwrap();

        assert_eq!(parsed, expected);
    }
}
