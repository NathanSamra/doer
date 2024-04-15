use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Priority {
    pub name: String,
    pub done: bool,
}

impl Priority {
    pub fn new(name: String) -> Self {
        Self { name, done: false }
    }
}
