use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub type SharedTask = Rc<RefCell<Task>>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    name: String,
    is_done: bool,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_done: false,
        }
    }

    pub fn set_done(&mut self, is_done: bool) {
        self.is_done = is_done;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut text = self.name.clone();
        if self.is_done {
            text += " - done"
        }

        writeln!(f, "{}", text)
    }
}

pub fn make_shared_task(task: Task) -> SharedTask {
    Rc::new(RefCell::new(task))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_not_done() {
        let task = Task::new("A task".to_string());
        assert!(!task.is_done);
    }

    #[test]
    fn set_done_true() {
        let mut task = Task::new("A task".to_string());
        task.set_done(true);
        assert!(task.is_done);
    }

    #[test]
    fn set_done_false() {
        let mut task = Task::new("A task".to_string());
        task.set_done(true);
        assert!(!task.is_done);
    }
}
