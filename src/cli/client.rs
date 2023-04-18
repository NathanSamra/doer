use crate::database::edit_day_guard::EditDayGuard;
use crate::database::Database;
use crate::model::day::PriorityId;
use crate::today::today;
use chrono::NaiveDate;

pub struct Client {
    database: Database,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            database: Database::at_os().expect("Database initialisation failed"),
        }
    }
}

impl Client {
    pub fn show_context(&self) {
        println!("{}", self.database.context())
    }

    pub fn set_context(&mut self, context: String) {
        self.database.set_context(context);
    }

    pub fn list_contexts(&self) {
        for context in self.database.contexts() {
            println!("{}", context)
        }
    }

    pub fn copy(&mut self, _from: &NaiveDate, _to: &NaiveDate) {
        todo!()
    }

    pub fn show_focus(&self) {
        todo!()
    }

    pub fn set_focus(&mut self, _focus: &str) {
        todo!()
    }

    pub fn start_break(&mut self) {
        todo!()
    }

    pub fn end_break(&mut self) {
        todo!()
    }

    pub fn end_day(&mut self) {
        todo!()
    }

    pub fn note(&mut self, note: String) {
        let mut date_editor = self.edit_day_guard(today());
        date_editor.day.note(note);
    }

    pub fn plan(&mut self, _date: &NaiveDate) {
        todo!()
    }

    pub fn show(&self, date: &NaiveDate) {
        match self.database.get(date) {
            None => {
                println!("No entry for {}", date)
            }
            Some(day) => {
                println!("{}", date);
                println!("{}", day);
            }
        }
    }

    pub fn show_last(&self) {
        match self.database.most_recent_past() {
            None => {
                println!("No entries to show")
            }
            Some(date) => self.show(date),
        }
    }

    pub fn set_tick(&mut self, date: NaiveDate, priority: PriorityId, is_done: bool) {
        let mut date_editor = self.edit_day_guard(date);
        match date_editor.day.update_priority(priority, is_done) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        }
    }

    fn edit_day_guard(&mut self, date: NaiveDate) -> EditDayGuard {
        EditDayGuard::new(&mut self.database, date)
    }
}
