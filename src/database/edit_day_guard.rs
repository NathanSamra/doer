use crate::database::Database;
use crate::model::day::Day;
use chrono::NaiveDate;

pub struct EditDayGuard<'a> {
    database: &'a mut Database,
    date: NaiveDate,
    pub day: Day,
}

impl<'a> EditDayGuard<'a> {
    pub fn new(database: &'a mut Database, date: NaiveDate) -> Self {
        let day = match database.get(&date) {
            None => Day::default(),
            Some(day_ref) => day_ref.clone(),
        };

        Self {
            database,
            date,
            day,
        }
    }
}

impl Drop for EditDayGuard<'_> {
    fn drop(&mut self) {
        self.database.set(self.date, self.day.clone())
    }
}
