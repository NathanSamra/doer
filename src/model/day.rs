use crate::model::focus::Focus;
use crate::model::r#break::BreakError;
use crate::model::task::Task;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
#[allow(unused_imports)]
use std::fmt::{Debug, Display};

// TODO: Consider adding date to Day, and then maybe Year could be a list instead of a map.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Day {
    // TODO: Actually I don't think any data in Day should be public. It has too much logic going on.
    pub priorities: Vec<Task>,
    log: Vec<Focus>,
    // TODO: I think the end time system doesn't work, especially with the focuses and breaks system.
    // Maybe the schedule should be independent from Focus and instead reference relevant Focuses and Breaks.
    end_time: Option<NaiveDateTime>,
    notes: Vec<String>,
}

impl Day {
    pub fn focus(&self) -> Option<&Focus> {
        match self.log.last() {
            None => None,
            Some(focus) => Some(focus),
        }
    }

    // TODO: Maybe return Result to indicate if the focus was unchanged?
    pub fn set_focus(&mut self, focus_name: String) {
        match self.log.last_mut() {
            None => {
                self.log.push(Focus::now(focus_name));
            }
            Some(focus) => {
                if focus.name == focus_name {
                    return;
                }
                // TODO: This unwraps because it assumes the error is a result of the break not existing,
                // but that assumption may not always hold. Should come up with a better pattern.
                focus.end_break().unwrap();
                self.log.push(Focus::now(focus_name));
            }
        }
    }

    pub fn start_break(&mut self) -> Result<(), BreakError> {
        match self.log.last_mut() {
            None => Err(BreakError::NoFocus),
            Some(focus) => focus.start_break(),
        }
    }

    pub fn end_break(&mut self) -> Result<(), BreakError> {
        match self.log.last_mut() {
            None => Err(BreakError::NoFocus),
            Some(focus) => focus.end_break(),
        }
    }

    pub fn log(&self) -> &Vec<Focus> {
        &self.log
    }

    pub fn end(&mut self) {
        self.end_time = Some(Local::now().naive_local());
    }

    pub fn notes(&self) -> &Vec<String> {
        &self.notes
    }

    pub fn add_note(&mut self, note: String) {
        self.notes.push(note)
    }

    fn display_priorities(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.priorities.is_empty() {
            writeln!(f, "No priorities")?;
            return Ok(());
        }

        let focus = match self.focus() {
            None => "",
            Some(focus) => focus.name.as_str(),
        };

        writeln!(f, "Priorities:")?;
        for (i, priority) in self.priorities.iter().enumerate() {
            let priority_name = priority.name.as_str();
            let mut line = format!("{i}. {priority_name}");

            if focus == priority_name {
                line += "*";
            }

            if priority.done {
                line += " - done";
            }

            writeln!(f, "{}", line)?;
        }

        if !focus.is_empty() {
            writeln!(f, "\nFocus: {focus}")?;
        }

        Ok(())
    }

    fn display_log(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.log().is_empty() {
            writeln!(f, "No log")?;
            return Ok(());
        }

        writeln!(f, "Log:")?;
        for focus in self.log() {
            let start = focus.start.format("%H:%M");
            let focus_name = &focus.name;
            // TODO: Instead of putting the formatting in here, why not impl Display for Focus.
            // This could apply to all the other structs as well.
            writeln!(f, "{start} - {focus_name}")?;

            for break_ in focus.breaks.iter() {
                let break_start = break_.start.format("%H:%M");

                let break_end = match break_.end {
                    None => "N/A".to_string(),
                    Some(end) => end.format("%H:%M").to_string(),
                };

                writeln!(f, "\t{break_start} - {break_end}")?;
            }
        }

        Ok(())
    }

    fn display_notes(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.notes().is_empty() {
            writeln!(f, "No notes")?;
            return Ok(());
        }

        writeln!(f, "Notes:")?;
        for (i, note) in self.notes().iter().enumerate() {
            writeln!(f, "{i}. {note}")?;
        }

        Ok(())
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.display_priorities(f)?;
        writeln!(f)?;
        self.display_log(f)?;
        writeln!(f)?;
        self.display_notes(f)?;
        Ok(())
    }
}
