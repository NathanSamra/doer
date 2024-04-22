use crate::model::log::{EntryType, Log, LogError};
use crate::model::task::{Task, TaskId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use thiserror::Error;

pub type Tasks = HashMap<TaskId, Task>;

// TODO: Consider using references to Task instead of TaskId.
// TODO: Consider adding date to Day, and then maybe Year could be a list instead of a map.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Day {
    tasks: Tasks,
    priorities: Vec<TaskId>,
    log: Log,
    notes: Vec<String>,
}

impl Day {
    pub fn tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect::<Vec<&Task>>()
    }

    pub fn get_task(&self, task_id: &TaskId) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    pub fn insert_task(&mut self, task: Task) -> Option<Task> {
        self.tasks.insert(task.id, task)
    }

    // TODO: I feel like just having a getter and setter is lazy. More encapsulation is needed.
    pub fn priorities(&self) -> &Vec<TaskId> {
        &self.priorities
    }

    pub fn set_priorities(&mut self, priorities: Vec<TaskId>) {
        if !priorities
            .iter()
            .all(|task_id| self.tasks.contains_key(task_id))
        {
            panic!("Not all priority tasks are in the day");
        }

        self.priorities = priorities;
    }

    pub fn extra_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter_map(|(task_id, _)| {
                if !self.priorities.contains(task_id) {
                    Some(&self.tasks[task_id])
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn focus(&self) -> Option<TaskId> {
        match self.log.last()?.type_ {
            EntryType::Focus(task_id) => Some(task_id),
            _ => None,
        }
    }

    pub fn start_day(&mut self) -> Result<(), LogError> {
        self.log.start_day()
    }

    // TODO: Maybe return Result to indicate if the focus was unchanged?
    pub fn start_focus(&mut self, task_id: TaskId) -> Result<(), DayError> {
        if !self.tasks.contains_key(&task_id) {
            return Err(DayError::TaskDoesNotExistInDay);
        }
        Ok(self.log.start_focus(task_id)?)
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

    fn display_priorities(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.priorities.is_empty() {
            writeln!(f, "No priorities")?;
            return Ok(());
        }

        let focus = self.focus();

        writeln!(f, "Priorities:")?;
        for (i, task_id) in self.priorities.iter().enumerate() {
            let task = &self.tasks[task_id];
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
                let focus_task = &self.tasks[&task_id];
                writeln!(f, "\nFocus: {}", focus_task.name)?;
            }
        }

        Ok(())
    }

    fn display_log(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.log.is_empty() {
            writeln!(f, "No log")?;
            return Ok(());
        }

        writeln!(f, "Log:")?;
        for focus in self.log.entries() {
            let start = focus.start.format("%H:%M");
            let focus_name = match focus.type_ {
                EntryType::Focus(task_id) => {
                    let task = &self.tasks[&task_id];
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

    fn display_notes(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.notes.is_empty() {
            writeln!(f, "No notes")?;
            return Ok(());
        }

        writeln!(f, "Notes:")?;
        for (i, note) in self.notes.iter().enumerate() {
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

#[derive(Error, Debug)]
pub enum DayError {
    #[error("Task already exists in this day")]
    TaskAlreadyExistsInDay,
    #[error("Task does not exist in this day")]
    TaskDoesNotExistInDay,
    #[error("Log error: {0}")]
    LogError(LogError),
}

impl From<LogError> for DayError {
    fn from(value: LogError) -> Self {
        DayError::LogError(value)
    }
}
