use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

// TODO: Consider adding date to Day, and then maybe Year could be a list instead of a map.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Day {
    // TODO: Actually I don't think any data in Day should be public. It has too much logic going on.
    pub priorities: Vec<Priority>,
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

// TODO: Far too many structs in this file. Should move them all out to their own files.
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Priority {
    pub name: String,
    pub done: bool,
}

impl Priority {
    pub fn new(name: String) -> Self {
        Self { name, done: false }
    }
}

// TODO: Should probably log the end time here also
#[derive(Clone, Deserialize, Serialize)]
pub struct Focus {
    pub name: String,
    pub start: NaiveDateTime,
    pub breaks: Vec<Break>,
}

pub enum BreakError {
    NoBreak,
    BreakAlreadyEnded,
    LastBreakNotEnded,
    NoFocus,
}

impl Debug for BreakError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for BreakError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for BreakError {}

// TODO: I feel like a now() constructor would be useful to say Starting new focus NOW
impl Focus {
    pub fn now(name: String) -> Self {
        Self {
            name,
            start: Local::now().naive_local(),
            breaks: vec![],
        }
    }

    pub fn start_break(&mut self) -> Result<(), BreakError> {
        match self.breaks.last() {
            None => {}
            Some(last_break) => {
                if last_break.end.is_none() {
                    return Err(BreakError::LastBreakNotEnded);
                }
            }
        };

        self.breaks.push(Break::new(Local::now().naive_local()));
        Ok(())
    }

    pub fn end_break(&mut self) -> Result<(), BreakError> {
        match self.breaks.last_mut() {
            Some(break_) => break_.end(),
            None => Err(BreakError::NoBreak),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Break {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

impl Break {
    pub fn new(start: NaiveDateTime) -> Self {
        Self { start, end: None }
    }

    pub fn end(&mut self) -> Result<(), BreakError> {
        match &mut self.end {
            Some(_) => {
                self.end = Some(Local::now().naive_local());
                Ok(())
            }
            None => Err(BreakError::BreakAlreadyEnded),
        }
    }
}
