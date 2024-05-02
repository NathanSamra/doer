use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type TaskId = Uuid;

// TODO: Impl Display?
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    // TODO: Should be a state: todo, doing, waiting, done.
    pub done: bool,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            done: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_not_done() {
        let task = Task::new("Test".to_string());
        assert!(!task.done);
    }

    #[test]
    fn new_has_name() {
        let task = Task::new("Test".to_string());
        assert_eq!(task.name, "Test");
    }

    #[test]
    fn new_has_id() {
        let task = Task::new("Test".to_string());
        assert_ne!(task.id, Uuid::nil());
    }
}
