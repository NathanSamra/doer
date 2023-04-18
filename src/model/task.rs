use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub type SharedTask = Rc<RefCell<Task>>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    pub name: String,
    pub is_done: bool,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_done: false,
        }
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
