use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type TaskId = Uuid;

// TODO: Impl Display?
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
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
