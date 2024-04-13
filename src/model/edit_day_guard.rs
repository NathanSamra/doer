use crate::model::data::Data;
use crate::model::day::Day;
use chrono::NaiveDate;

pub struct EditDayGuard<'a> {
    data: &'a mut Data,
    date: NaiveDate,
    day: Day,
}

impl<'a> EditDayGuard<'a> {
    pub fn new(date: NaiveDate, data: &'a mut Data) -> Self {
        let day = data.day(&date);
        Self { date, data, day }
    }

    pub fn day(&mut self) -> &mut Day {
        &mut self.day
    }
}

impl<'a> Drop for EditDayGuard<'a> {
    fn drop(&mut self) {
        self.data.set_day(&self.date, &self.day)
    }
}
