use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    pub name: String,
    pub is_done: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SharedTask(Rc<RefCell<Task>>);

impl<'de> Deserialize<'de> for SharedTask {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl Serialize for SharedTask {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}
