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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tasks_is_empty_when_empty() {
        let day = Day::default();
        assert!(day.tasks().is_empty());
    }

    #[test]
    fn tasks_has_task_when_inserted() {
        let mut day = Day::default();
        let task = Task::new("Task".to_string());
        day.insert_task(task.clone());
        assert_eq!(day.tasks(), vec![&task]);
    }

    #[test]
    fn tasks_has_tasks_when_inserted() {
        let mut day = Day::default();
        let task1 = Task::new("Task1".to_string());
        let task2 = Task::new("Task2".to_string());
        day.insert_task(task1.clone());
        day.insert_task(task2.clone());
        assert_eq!(day.tasks(), vec![&task2, &task1]);
    }

    #[test]
    fn get_task_is_none_when_empty() {
        let day = Day::default();
        assert!(day.get_task(&TaskId::now_v7()).is_none());
    }

    #[test]
    fn get_task_is_none_when_task_does_not_exist() {
        let mut day = Day::default();
        let task = Task::new("Task".to_string());
        day.insert_task(task);
        assert!(day.get_task(&TaskId::now_v7()).is_none());
    }

    #[test]
    fn get_task_is_some_when_task_exists() {
        let mut day = Day::default();
        let task = Task::new("Task".to_string());
        let task_id = task.id;
        day.insert_task(task);
        assert_eq!(day.get_task(&task_id).unwrap().name, "Task");
    }

    #[test]
    fn insert_task_returns_none_when_empty() {
        let mut day = Day::default();
        let task = Task::new("Task".to_string());
        assert!(day.insert_task(task).is_none());
    }

    #[test]
    fn insert_task_returns_none_when_task_does_not_exist() {
        let mut day = Day::default();
        let task1 = Task::new("Task1".to_string());
        day.insert_task(task1);
        let task2 = Task::new("Task2".to_string());
        assert!(day.insert_task(task2).is_none());
    }

    #[test]
    fn insert_task_when_task_exists_returns_some_and_updates_task() {
        let mut day = Day::default();
        let task1 = Task::new("Task1".to_string());
        let id = task1.id;
        day.insert_task(task1);
        let mut task2 = day.get_task(&id).unwrap().clone();
        task2.name = "Task2".to_string();
        let replaced_task = day.insert_task(task2).unwrap();
        assert_eq!(replaced_task.id, id);
        assert_eq!(day.get_task(&id).unwrap().name, "Task2");
    }

    #[test]
    fn priorities_is_empty_when_empty() {
        let day = Day::default();
        assert!(day.priorities().is_empty());
    }

    #[test]
    fn set_priorities_sets_priorities() {
        let mut day = Day::default();
        let task1 = Task::new("Task1".to_string());
        let task2 = Task::new("Task2".to_string());
        day.insert_task(task1.clone());
        day.insert_task(task2.clone());
        day.set_priorities(vec![task1.id, task2.id]);
        assert_eq!(day.priorities(), &vec![task1.id, task2.id]);
    }

    #[test]
    #[should_panic]
    fn set_priorities_to_non_existant_tasks_panics() {
        let mut day = Day::default();
        let task_id1 = TaskId::now_v7();
        let task_id2 = TaskId::now_v7();
        day.set_priorities(vec![task_id1, task_id2]);
    }

    #[test]
    fn extra_tasks_is_empty_when_empty() {
        let day = Day::default();
        assert!(day.extra_tasks().is_empty());
    }

    #[test]
    fn extra_tasks_is_empty_when_all_priorities() {
        let mut day = Day::default();
        let task1 = Task::new("Task1".to_string());
        let task2 = Task::new("Task2".to_string());
        day.insert_task(task1.clone());
        day.insert_task(task2.clone());
        day.set_priorities(vec![task1.id, task2.id]);
        assert!(day.extra_tasks().is_empty());
    }

    #[test]
    fn extra_tasks_has_tasks_when_not_all_priorities() {
        let mut day = Day::default();
        let task1 = Task::new("Task1".to_string());
        let task2 = Task::new("Task2".to_string());
        let task3 = Task::new("Task3".to_string());
        day.insert_task(task1.clone());
        day.insert_task(task2.clone());
        day.insert_task(task3.clone());
        day.set_priorities(vec![task1.id, task2.id]);
        assert_eq!(day.extra_tasks(), vec![&task3]);
    }

    #[test]
    fn log_is_empty_when_empty() {
        let day = Day::default();
        assert!(day.log().is_empty());
    }

    #[test]
    fn log_is_not_empty_when_started() {
        let mut day = Day::default();
        day.start_day().unwrap();
        assert!(!day.log().is_empty());
    }

    #[test]
    fn focus_is_none_when_empty() {
        let day = Day::default();
        assert!(day.focus().is_none());
    }

    #[test]
    fn focus_is_none_when_not_focused() {
        let mut day = Day::default();
        day.start_day().unwrap();
        assert!(day.focus().is_none());
    }

    #[test]
    fn focus_is_some_when_focused() {
        let mut day = Day::default();
        let task = Task::new("Task".to_string());
        day.insert_task(task.clone());
        day.start_day().unwrap();
        day.start_focus(task.id).unwrap();
        assert_eq!(day.focus().unwrap(), task.id);
    }

    #[test]
    fn start_day_starts_day() {
        let mut day = Day::default();
        day.start_day().unwrap();
        assert!(!day.log().is_empty());
    }

    #[test]
    fn start_focus_starts_focus() {
        let mut day = Day::default();
        let task = Task::new("Task".to_string());
        day.insert_task(task.clone());
        day.start_day().unwrap();
        day.start_focus(task.id).unwrap();
        assert_eq!(day.focus().unwrap(), task.id);
    }

    #[test]
    fn start_focus_returns_error_when_task_does_not_exist() {
        let mut day = Day::default();
        assert!(day.start_focus(TaskId::now_v7()).is_err());
    }

    #[test]
    fn start_break_starts_break() {
        let mut day = Day::default();
        let task = Task::new("Task".to_string());
        day.insert_task(task.clone());
        day.start_day().unwrap();
        day.start_focus(task.id).unwrap();
        day.start_break().unwrap();
        assert!(day.focus().is_none());
    }

    #[test]
    fn end_day_ends_day() {
        let mut day = Day::default();
        day.start_day().unwrap();
        day.end_day().unwrap();
        assert_eq!(day.log().last().unwrap().type_, EntryType::DayEnded);
    }

    #[test]
    fn notes_is_empty_when_empty() {
        let day = Day::default();
        assert!(day.notes().is_empty());
    }

    #[test]
    fn add_note_adds_note() {
        let mut day = Day::default();
        day.add_note("Note".to_string());
        assert_eq!(day.notes(), &vec!["Note".to_string()]);
    }

    #[test]
    fn add_two_notes_adds_notes() {
        let mut day = Day::default();
        day.add_note("Note1".to_string());
        day.add_note("Note2".to_string());
        assert_eq!(day.notes(), &vec!["Note1".to_string(), "Note2".to_string()]);
    }
}
