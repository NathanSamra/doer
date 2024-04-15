use crate::model::focus::Focus;
use crate::model::r#break::BreakError;
use crate::model::task::Task;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
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
}
