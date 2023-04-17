use crate::database::edit_day_guard::EditDayGuard;
use crate::database::DATABASE;
use crate::model::priority::PriorityId;
use chrono::NaiveDate;

use std::ops::DerefMut;

pub fn set_tick(date: NaiveDate, priority: PriorityId, is_done: bool) {
    let mut database = DATABASE.lock().unwrap();
    let mut date_editor = EditDayGuard::new(database.deref_mut(), date);
    match date_editor.day.update_priority(priority, is_done) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}
