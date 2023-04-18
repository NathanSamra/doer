use crate::database::edit_day_guard::EditDayGuard;
use crate::database::Database;
use crate::model::day::PriorityId;
use crate::today::today;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

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

    pub fn copy(&mut self, from: &NaiveDate, to: NaiveDate) -> Result<(), Error> {
        let priorities = match self.database.get(from) {
            None => {
                println!("No entry to copy from at {}", from);
                Err(Error::default())
            }
            Some(from_day) => Ok(from_day.priorities()),
        }?;

        let mut to_day_editor = self.edit_day_guard(to);
        match to_day_editor.day.set_priorities(priorities) {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Copy failed: {}", err);
                Err(Error::default())
            }
        }
    }

    pub fn show_focus(&self) {
        match self.database.get(&today()) {
            None => {
                println!("No focus");
            }
            Some(day) => match day.focus() {
                None => {
                    println!("No focus");
                }
                Some(focus) => {
                    println!("{}", focus)
                }
            },
        }
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

#[derive(Debug, Default)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}
