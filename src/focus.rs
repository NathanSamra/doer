use crate::priority::PriorityId;

#[derive(Clone)]
pub enum Focus {
    Priority(PriorityId),
    Other(String),
}

impl<'s> From<&'s str> for Focus {
    fn from(value: &'s str) -> Self {
        match value.parse::<PriorityId>() {
            Ok(priority_id) => Focus::Priority(priority_id),
            Err(_) => Focus::Other(value.to_string()),
        }
    }
}
