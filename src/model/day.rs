use crate::model::priority::{Priority, PriorityId};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Day {
    priorities: Vec<Priority>,
    notes: Vec<String>,
}

impl Day {
    pub fn note(&mut self, note: String) {
        self.notes.push(note)
    }

    pub fn update_priority(&mut self, priority_id: PriorityId, is_done: bool) -> Result<(), Error> {
        match self.priorities.get_mut(priority_id) {
            None => Err(Error::default()),
            Some(priority) => {
                priority.is_done = is_done;
                Ok(())
            }
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

#[derive(Debug, Default)]
pub struct Error;

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
