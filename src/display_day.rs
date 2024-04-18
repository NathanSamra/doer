use crate::database::Database;
use crate::model::day::Day;
use crate::model::log::EntryType;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

pub struct DayDisplayer<'a> {
    database: &'a Database,
    date: NaiveDate,
}

impl DayDisplayer<'_> {
    pub fn new(date: NaiveDate, database: &Database) -> DayDisplayer {
        DayDisplayer { database, date }
    }

    fn display_priorities(&self, day: &Day, f: &mut Formatter<'_>) -> std::fmt::Result {
        let priorities = day.priorities();
        if priorities.is_empty() {
            writeln!(f, "No priorities")?;
            return Ok(());
        }

        let focus = day.focus();

        writeln!(f, "Priorities:")?;
        for (i, task_id) in priorities.iter().enumerate() {
            let task = self
                .database
                .get_task(task_id)
                .expect("Task not found for ID");
            let priority_name = task.name.as_str();
            let mut line = format!("{i}. {priority_name}");

            match focus {
                None => {}
                Some(focus_id) => {
                    if &focus_id == task_id {
                        line += "*";
                    }
                }
            }

            if task.done {
                line += " - done";
            }

            writeln!(f, "{}", line)?;
        }

        match focus {
            None => {}
            Some(task_id) => {
                let focus_task = self
                    .database
                    .get_task(&task_id)
                    .expect("Task not found for ID");
                writeln!(f, "\nFocus: {}", focus_task.name)?;
            }
        }

        Ok(())
    }

    fn display_log(&self, day: &Day, f: &mut Formatter<'_>) -> std::fmt::Result {
        let log = day.log();
        if log.is_empty() {
            writeln!(f, "No log")?;
            return Ok(());
        }

        writeln!(f, "Log:")?;
        for focus in log.entries() {
            let start = focus.start.format("%H:%M");
            let focus_name = match focus.type_ {
                EntryType::Focus(task_id) => {
                    let task = self
                        .database
                        .get_task(&task_id)
                        .expect("Task not found for ID");
                    task.name.as_str()
                }
                EntryType::Unfocused => "Unfocused",
                EntryType::DayEnded => "Day end",
            };

            // TODO: Instead of putting the formatting in here, why not impl Display for Focus.
            // This could apply to all the other structs as well.
            writeln!(f, "{start} - {focus_name}")?;
        }

        Ok(())
    }

    fn display_notes(&self, day: &Day, f: &mut Formatter<'_>) -> std::fmt::Result {
        let notes = day.notes();
        if notes.is_empty() {
            writeln!(f, "No notes")?;
            return Ok(());
        }

        writeln!(f, "Notes:")?;
        for (i, note) in notes.iter().enumerate() {
            writeln!(f, "{i}. {note}")?;
        }

        Ok(())
    }
}

impl Display for DayDisplayer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let day = self.database.get_day(&self.date);
        self.display_priorities(&day, f)?;
        writeln!(f)?;
        self.display_log(&day, f)?;
        writeln!(f)?;
        self.display_notes(&day, f)?;
        Ok(())
    }
}
