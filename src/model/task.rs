use serde::{Deserialize, Serialize};

// TODO: Impl Display?
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self { name, done: false }
    }
}
