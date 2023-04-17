use crate::database::edit_day_guard::EditDayGuard;
use crate::database::Database;
use crate::today::today;

pub fn note(database: &mut Database, note: String) {
    let mut date_editor = EditDayGuard::new(database, today());
    date_editor.day.note(note);
}
