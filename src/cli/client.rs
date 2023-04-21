use crate::database::edit_day_guard::EditDayGuard;
use crate::database::Database;
use crate::model::day::PriorityId;
use crate::model::{day, focus};
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
            None => Err(Error::MissingDay(*from)),
            Some(from_day) => Ok(from_day.priorities()),
        }?;

        let mut to_day_editor = self.edit_day_guard(to);
        to_day_editor.day.set_priorities(priorities)?;
        Ok(())
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

    pub fn set_focus(&mut self, focus_str: String) -> Result<(), Error> {
        let mut day_editor = self.edit_day_guard(today());

        match focus_str.parse::<PriorityId>() {
            Ok(priority_id) => {
                day_editor.day.set_focus_from_priority(&priority_id)?;
                Ok(())
            }
            Err(_) => {
                day_editor.day.set_focus(focus_str);
                Ok(())
            }
        }
    }

    pub fn start_break(&mut self) -> Result<(), Error> {
        let mut day_editor = self.edit_day_guard(today());

        match day_editor.day.focus_mut() {
            None => Err(Error::NoFocus),
            Some(focus) => {
                focus.start_break()?;
                Ok(())
            }
        }
    }

    pub fn end_break(&mut self) -> Result<(), Error> {
        let mut day_editor = self.edit_day_guard(today());

        match day_editor.day.focus_mut() {
            None => Err(Error::NoFocus),
            Some(focus) => {
                focus.end_break()?;
                Ok(())
            }
        }
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

    pub fn set_tick(
        &mut self,
        date: NaiveDate,
        priority: PriorityId,
        is_done: bool,
    ) -> Result<(), Error> {
        let mut date_editor = self.edit_day_guard(date);
        let mut priority = date_editor.day.priority_mut(priority)?;
        priority.set_done(is_done);
        Ok(())
    }

    fn edit_day_guard(&mut self, date: NaiveDate) -> EditDayGuard {
        EditDayGuard::new(&mut self.database, date)
    }
}

#[derive(Debug)]
pub enum Error {
    MissingDay(NaiveDate),
    NoFocus,
    Focus(focus::Error),
    Day(day::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingDay(date) => {
                writeln!(f, "No entry to copy from at {}", date)
            }
            Error::NoFocus => {
                writeln!(f, "No focus set")
            }
            Error::Focus(err) => {
                writeln!(f, "{}", err)
            }
            Error::Day(err) => {
                writeln!(f, "{}", err)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<focus::Error> for Error {
    fn from(value: focus::Error) -> Self {
        Error::Focus(value)
    }
}

impl From<day::Error> for Error {
    fn from(value: day::Error) -> Self {
        Error::Day(value)
    }
}
