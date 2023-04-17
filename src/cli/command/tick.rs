use crate::database::edit_day_guard::EditDayGuard;
use crate::database::Database;
use crate::model::day::PriorityId;
use chrono::NaiveDate;

pub fn set_tick(database: &mut Database, date: NaiveDate, priority: PriorityId, is_done: bool) {
    let mut date_editor = EditDayGuard::new(database, date);
    match date_editor.day.update_priority(priority, is_done) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}
