use crate::database::TaskId;
use crate::model::log::{EntryType, Log, LogError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

// TODO: Consider using references to Task instead of TaskId.
// TODO: Consider adding date to Day, and then maybe Year could be a list instead of a map.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Day {
    priorities: Vec<TaskId>,
    extra_tasks: HashSet<TaskId>,
    log: Log,
    notes: Vec<String>,
}

impl Day {
    pub fn tasks(&self) -> Vec<TaskId> {
        self.priorities
            .iter()
            .chain(self.extra_tasks.iter())
            .cloned()
            .collect()
    }

    // TODO: I feel like just having a getter and setter is lazy. More encapsulation is needed.
    pub fn priorities(&self) -> &Vec<TaskId> {
        &self.priorities
    }

    pub fn set_priorities(&mut self, priorities: Vec<TaskId>) {
        for task_id in self.priorities.iter() {
            if !priorities.contains(task_id) {
                self.extra_tasks.insert(*task_id);
            }
        }

        self.priorities = priorities;

        for task_id in self.priorities.iter() {
            self.extra_tasks.remove(task_id);
        }
    }

    pub fn extra_tasks(&self) -> &HashSet<TaskId> {
        &self.extra_tasks
    }

    pub fn add_task(&mut self, task_id: TaskId) -> Result<(), DayError> {
        if self.tasks().contains(&task_id) {
            return Err(DayError::TaskAlreadyExistsInDay);
        }
        self.extra_tasks.insert(task_id);
        Ok(())
    }

    pub fn focus(&self) -> Option<TaskId> {
        match self.log.last()?.type_ {
            EntryType::Focus(task_id) => Some(task_id),
            _ => None,
        }
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn start_day(&mut self) -> Result<(), LogError> {
        self.log.start_day()
    }

    // TODO: Maybe return Result to indicate if the focus was unchanged?
    pub fn start_focus(&mut self, task_id: TaskId) -> Result<(), LogError> {
        self.log.start_focus(task_id)
    }

    pub fn start_break(&mut self) -> Result<(), LogError> {
        self.log.start_unfocused()
    }

    pub fn end_day(&mut self) -> Result<(), LogError> {
        self.log.end_day()
    }

    pub fn notes(&self) -> &Vec<String> {
        &self.notes
    }

    pub fn add_note(&mut self, note: String) {
        self.notes.push(note)
    }
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("Task already exists in this day")]
    TaskAlreadyExistsInDay,
}
