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
                    let day_ended = self.entries.pop().unwrap();
                    let last_entry = self.entries.last().unwrap();
                    match last_entry.type_ {
                        EntryType::Focus(_) => {
                            self.entries.push(day_ended.to_unfocused());
                            Ok(())
                        }
                        EntryType::Unfocused => Ok(()),
                        EntryType::DayEnded => {
                            panic!("Day ended twice");
                        }
                    }
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
                    let day_ended = self.entries.pop().unwrap();
                    let last_entry = self.entries.last().unwrap();
                    match last_entry.type_ {
                        EntryType::Focus(_) => {
                            self.entries.push(day_ended.to_unfocused());
                        }
                        EntryType::Unfocused => {}
                        EntryType::DayEnded => {
                            panic!("Day ended twice");
                        }
                    };

                    self.entries.push(Entry::now(EntryType::Focus(task_id)));
                    Ok(())
                }
            },
        }
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
                EntryType::DayEnded => Err(LogError::DayAlreadyEnded),
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

    pub fn to_unfocused(&self) -> Self {
        Self {
            type_: EntryType::Unfocused,
            start: self.start,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_has_no_entries() {
        let log = Log::default();
        assert!(log.entries().is_empty());
    }

    #[test]
    fn new_is_empty() {
        let log = Log::default();
        assert!(log.is_empty());
    }

    #[test]
    fn start_day_is_not_empty() {
        let mut log = Log::default();
        log.start_day().unwrap();
        assert!(!log.is_empty());
    }

    #[test]
    fn start_day_is_unfocused() {
        let mut log = Log::default();
        log.start_day().unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused]
        );
    }

    #[test]
    fn start_day_twice_fails() {
        let mut log = Log::default();
        log.start_day().unwrap();
        assert!(log.start_day().is_err());
    }

    #[test]
    fn start_unfocused_after_start_day_fails() {
        let mut log = Log::default();
        log.start_day().unwrap();
        assert!(log.start_unfocused().is_err());
    }

    #[test]
    fn start_unfocused_without_start_day() {
        let mut log = Log::default();
        log.start_unfocused().unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused]
        );
    }

    #[test]
    fn start_two_unfocused_fails() {
        let mut log = Log::default();
        log.start_unfocused().unwrap();
        assert!(log.start_unfocused().is_err());
    }

    #[test]
    fn start_focus_after_start_day() {
        let mut log = Log::default();
        let task_id = TaskId::default();
        log.start_day().unwrap();
        log.start_focus(task_id).unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused, EntryType::Focus(task_id)]
        );
    }

    #[test]
    fn start_focus_without_start_day() {
        let mut log = Log::default();
        let task_id = TaskId::default();
        log.start_focus(task_id).unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Focus(task_id)]
        );
    }

    #[test]
    fn start_focus_after_start_unfocused() {
        let mut log = Log::default();
        let task_id = TaskId::default();
        log.start_unfocused().unwrap();
        log.start_focus(task_id).unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused, EntryType::Focus(task_id)]
        );
    }

    #[test]
    fn start_the_same_focus_twice_fails() {
        let mut log = Log::default();
        let task_id = TaskId::default();
        log.start_focus(task_id).unwrap();
        assert!(log.start_focus(task_id).is_err());
    }

    #[test]
    fn start_two_different_focuses() {
        let mut log = Log::default();
        let task_id1 = TaskId::now_v7();
        let task_id2 = TaskId::now_v7();
        log.start_focus(task_id1).unwrap();
        log.start_focus(task_id2).unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Focus(task_id1), EntryType::Focus(task_id2)]
        );
    }

    #[test]
    fn end_day_after_start_day() {
        let mut log = Log::default();
        log.start_day().unwrap();
        log.end_day().unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused, EntryType::DayEnded]
        );
    }

    #[test]
    fn end_day_without_start_day_fails() {
        let mut log = Log::default();
        assert!(log.end_day().is_err());
    }

    #[test]
    fn end_day_after_start_unfocused() {
        let mut log = Log::default();
        log.start_unfocused().unwrap();
        log.end_day().unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused, EntryType::DayEnded]
        );
    }

    #[test]
    fn end_day_after_start_focus() {
        let mut log = Log::default();
        let task_id = TaskId::default();
        log.start_focus(task_id).unwrap();
        log.end_day().unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Focus(task_id), EntryType::DayEnded]
        );
    }

    #[test]
    fn end_day_twice_fails() {
        let mut log = Log::default();
        log.start_day().unwrap();
        log.end_day().unwrap();
        assert!(log.end_day().is_err());
    }

    #[test]
    fn start_day_after_end_day_fails() {
        let mut log = Log::default();
        log.start_day().unwrap();
        log.end_day().unwrap();
        assert!(log.start_day().is_err());
    }

    #[test]
    fn unfocused_after_end_day() {
        let mut log = Log::default();
        log.start_day().unwrap();
        log.end_day().unwrap();
        log.start_unfocused().unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused]
        );
    }

    #[test]
    fn unfocused_after_focus_then_end_day() {
        let mut log = Log::default();
        let task_id = TaskId::default();
        log.start_focus(task_id).unwrap();
        log.end_day().unwrap();
        log.start_unfocused().unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Focus(task_id), EntryType::Unfocused]
        );
    }

    #[test]
    fn focus_after_end_day() {
        let mut log = Log::default();
        let task_id = TaskId::default();
        log.start_day().unwrap();
        log.end_day().unwrap();
        log.start_focus(task_id).unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[EntryType::Unfocused, EntryType::Focus(task_id)]
        );
    }

    #[test]
    fn different_focus_after_end_day() {
        let mut log = Log::default();
        let task_id1 = TaskId::now_v7();
        let task_id2 = TaskId::now_v7();
        log.start_focus(task_id1).unwrap();
        log.end_day().unwrap();
        log.start_focus(task_id2).unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[
                EntryType::Focus(task_id1),
                EntryType::Unfocused,
                EntryType::Focus(task_id2)
            ]
        );
    }

    #[test]
    fn the_same_focus_after_end_day() {
        let mut log = Log::default();
        let task_id = TaskId::now_v7();
        log.start_focus(task_id).unwrap();
        log.end_day().unwrap();
        log.start_focus(task_id).unwrap();
        assert_eq!(
            log.entries()
                .iter()
                .map(|entry| entry.type_.clone())
                .collect::<Vec<EntryType>>(),
            &[
                EntryType::Focus(task_id),
                EntryType::Unfocused,
                EntryType::Focus(task_id)
            ]
        );
    }
}
