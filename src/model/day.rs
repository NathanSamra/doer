use crate::model::focus;
use crate::model::focus::Focus;
use crate::model::task::{make_shared_task, SharedTask, Task};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub type PriorityId = usize;

const MAX_PRIORITIES: usize = 6;

#[derive(Clone, Debug, Default)]
pub struct Day {
    priorities: Vec<SharedTask>,
    focuses: Vec<Focus>,
    notes: Vec<String>,
}

impl Day {
    pub fn focus(&self) -> Option<&Focus> {
        self.focuses.last()
    }

    #[allow(dead_code)]
    pub fn notes(&self) -> &Vec<String> {
        &self.notes
    }

    pub fn priorities(&self) -> Vec<Task> {
        self.priorities
            .iter()
            .map(|p| p.borrow().to_owned())
            .collect()
    }

    pub fn note(&mut self, note: String) {
        self.notes.push(note)
    }

    pub fn update_priority(&mut self, priority_id: PriorityId, is_done: bool) -> Result<(), Error> {
        match self.priorities.get(priority_id) {
            None => Err(Error::InvalidPriorityId),
            Some(priority) => {
                priority.borrow_mut().set_done(is_done);
                Ok(())
            }
        }
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

    pub fn start_break(&mut self) -> Result<(), Error> {
        let focus = self.cur_focus_mut()?;

        match focus.start_break() {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Focus(err)),
        }
    }

    pub fn end_break(&mut self) -> Result<(), Error> {
        let focus = self.cur_focus_mut()?;

        match focus.end_break() {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Focus(err)),
        }
    }

    fn cur_focus_mut(&mut self) -> Result<&mut Focus, Error> {
        match self.focuses.last_mut() {
            None => Err(Error::NoFocusSet),
            Some(focus) => Ok(focus),
        }
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
    NoFocusSet,
    Focus(focus::Error),
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
