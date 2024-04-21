use crate::model::task::TaskId;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Log {
    entries: Vec<Entry>,
}

impl Log {
    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn last(&self) -> Option<&Entry> {
        self.entries.last()
    }

    pub fn start_day(&mut self) -> Result<(), LogError> {
        if !self.entries.is_empty() {
            return Err(LogError::DayAlreadyStarted);
        }

        self.entries.push(Entry::now(EntryType::Unfocused));
        Ok(())
    }

    pub fn end_day(&mut self) -> Result<(), LogError> {
        match self.entries.last() {
            None => Err(LogError::DayNeverStarted),
            Some(entry) => match entry.type_ {
                EntryType::Focus { .. } => {
                    self.entries.push(Entry::now(EntryType::DayEnded));
                    Ok(())
                }
                EntryType::Unfocused => {
                    self.entries.push(Entry::now(EntryType::DayEnded));
                    Ok(())
                }
                EntryType::DayEnded => Ok(()),
            },
        }
    }

    pub fn start_unfocused(&mut self) -> Result<(), LogError> {
        match self.entries.last() {
            None => {
                self.entries.push(Entry::now(EntryType::Unfocused));
                Ok(())
            }
            Some(entry) => match entry.type_ {
                EntryType::Focus { .. } => {
                    self.entries.push(Entry::now(EntryType::Unfocused));
                    Ok(())
                }
                EntryType::Unfocused => Err(LogError::AlreadyUnfocused),
                EntryType::DayEnded => {
                    self.entries.pop();
                    self.entries.push(Entry::now(EntryType::Unfocused));
                    Ok(())
                }
            },
        }
    }

    pub fn start_focus(&mut self, task_id: TaskId) -> Result<(), LogError> {
        match self.entries.last() {
            None => {
                self.entries.push(Entry::now(EntryType::Focus(task_id)));
                Ok(())
            }
            Some(entry) => match entry.type_ {
                EntryType::Focus(current_focus_id) => {
                    if current_focus_id == task_id {
                        Err(LogError::AlreadyFocusedOnFocus)
                    } else {
                        self.entries.push(Entry::now(EntryType::Focus(task_id)));
                        Ok(())
                    }
                }
                EntryType::Unfocused => {
                    self.entries.push(Entry::now(EntryType::Focus(task_id)));
                    Ok(())
                }
                EntryType::DayEnded => {
                    self.entries.pop();
                    self.entries.push(Entry::now(EntryType::Focus(task_id)));
                    Ok(())
                }
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Entry {
    pub type_: EntryType,
    pub start: NaiveDateTime,
}

impl Entry {
    pub fn now(type_: EntryType) -> Self {
        Self {
            type_,
            start: Local::now().naive_local(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum EntryType {
    Focus(TaskId),
    Unfocused,
    DayEnded,
}

#[derive(Error, Debug)]
pub enum LogError {
    #[error("Day is already started")]
    DayAlreadyStarted,
    #[error("Already unfocused")]
    AlreadyUnfocused,
    #[error("Day already ended")]
    DayAlreadyEnded,
    #[error("Day never started")]
    DayNeverStarted,
    #[error("Already focused on that focus")]
    AlreadyFocusedOnFocus,
}
