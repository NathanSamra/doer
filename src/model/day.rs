use crate::model::focus::Focus;
use crate::model::task::{make_shared_task, SharedTask, Task};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub type PriorityId = usize;

const MAX_PRIORITIES: usize = 6;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Day {
    priorities: Vec<SharedTask>,
    focuses: Vec<Focus>,
    notes: Vec<String>,
}

impl Day {
    pub fn focus(&self) -> Option<&Focus> {
        self.focuses.last()
    }

    pub fn focus_mut(&mut self) -> Option<&mut Focus> {
        self.focuses.last_mut()
    }

    pub fn set_focus(&mut self, focus_str: String) {
        let task = make_shared_task(Task::new(focus_str));
        let focus = Focus::new(task);
        self.focuses.push(focus);
    }

    pub fn set_focus_from_priority(&mut self, priority_id: &PriorityId) -> Result<(), Error> {
        let task = match self.priorities.get(*priority_id) {
            None => Err(Error::InvalidPriorityId),
            Some(priority) => Ok(priority.clone()),
        }?;

        let focus = Focus::new(task);
        self.focuses.push(focus);
        Ok(())
    }

    pub fn priority_mut(&mut self, priority_id: PriorityId) -> Result<RefMut<Task>, Error> {
        match self.priorities.get(priority_id) {
            None => Err(Error::InvalidPriorityId),
            Some(priority) => Ok(priority.borrow_mut()),
        }
    }

    pub fn priorities(&self) -> Vec<Task> {
        self.priorities
            .iter()
            .map(|p| p.borrow().to_owned())
            .collect()
    }

    pub fn set_priorities(&mut self, priorities: Vec<Task>) -> Result<(), Error> {
        if priorities.len() > MAX_PRIORITIES {
            return Err(Error::TooManyPriorities);
        }

        self.priorities = priorities
            .into_iter()
            .map(|p| Rc::new(RefCell::new(p)))
            .collect();
        Ok(())
    }

    pub fn note(&mut self, note: String) {
        self.notes.push(note)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.notes.is_empty() {
            writeln!(f, "No notes")?
        }

        writeln!(f, "Notes:")?;
        for (i, note) in self.notes.iter().enumerate() {
            writeln!(f, "{}. {}", i, note)?
        }

        Ok(())
    }
}

impl Serialize for Day {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Day {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Debug)]
pub enum Error {
    TooManyPriorities,
    InvalidPriorityId,
}

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note() {
        let mut day = Day::default();
        day.note("A note".to_string());

        let expected = vec!["A note".to_string()];

        assert_eq!(day.notes, expected)
    }
}
