use crate::model::day::Day;
use crate::model::task::Task;
use crate::storage::storage_handler::StorageHandler;
use chrono::NaiveDate;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

pub type TaskId = Uuid;
pub type Tasks = HashMap<TaskId, Task>;
pub type Days = HashMap<NaiveDate, Day>;

// TODO: Put context into this struct so it can handle re-loading itself if the context changes.
// TODO: Need to decide what to do if actions are no-op (i.e already existed). Maybe we return a bool as a less extreme response than a Result.
// TODO: To save memory should avoid populating all tasks and days from
// old years/done or lost tasks. A task is lost when it is no longer
// accessible? When crossing to a new year maybe tasks should have a state saying Done/Not Done/Not Done and passed on to next year.
// Or maybe we just add a prune() method that removes all completed/lost tasks from a defined year.
pub struct Database {
    storage: StorageHandler,
    tasks: Tasks,
    days: Days,
}

impl Database {
    pub fn load(_storage: StorageHandler) -> Self {
        todo!()
    }

    pub fn context(&self) -> String {
        self.storage.context()
    }

    pub fn contexts(&self) -> Vec<String> {
        self.storage.contexts()
    }

    pub fn set_context(&mut self, _context: String) {
        // TODO: Reload data
        todo!()
    }

    pub fn new_context(&mut self, _context: String) {
        todo!()
    }

    pub fn last_date(&self) -> Option<NaiveDate> {
        todo!()
    }

    pub fn days(&self) -> &HashMap<NaiveDate, Day> {
        &self.days
    }

    pub fn add_task(&mut self, task: Task) -> TaskId {
        let id = Uuid::now_v7();
        self.tasks.insert(id, task);
        id
    }

    pub fn get_task(&self, id: &TaskId) -> Result<&Task, DatabaseError> {
        self.tasks.get(id).ok_or(DatabaseError::NoTaskForId)
    }

    pub fn set_task(&mut self, id: TaskId, task: Task) -> Result<(), DatabaseError> {
        match self.tasks.get_mut(&id) {
            Some(t) => {
                *t = task;
                Ok(())
            }
            None => Err(DatabaseError::NoTaskForId),
        }
    }

    pub fn set_day(&mut self, date: NaiveDate, day: Day) {
        self.days.insert(date, day);
    }

    pub fn get_day(&self, date: &NaiveDate) -> Day {
        match self.days.get(date) {
            None => Day::default(),
            Some(day) => day.clone(),
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("No task for given ID")]
    NoTaskForId,
}
