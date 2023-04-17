use serde::{Deserialize, Serialize};

pub type PriorityId = usize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Priority {
    pub name: String,
    pub is_done: bool,
}
