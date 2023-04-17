use crate::database::edit_day_guard::EditDayGuard;
use crate::database::DATABASE;
use crate::today::today;
use std::ops::DerefMut;

pub fn note(note: String) {
    let mut database = DATABASE.lock().unwrap();
    let mut date_editor = EditDayGuard::new(database.deref_mut(), today());
    date_editor.day.note(note);
}
