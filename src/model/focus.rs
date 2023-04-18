use crate::model::task::SharedTask;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Focus {
    #[allow(dead_code)]
    task: SharedTask,
}

impl Focus {
    pub fn new(task: SharedTask) -> Self {
        Self { task }
    }
}

impl Display for Focus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let task = self.task.borrow();
        writeln!(f, "{}", task)
    }
}
