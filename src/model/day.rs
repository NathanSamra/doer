use crate::model::focus::Focus;
use crate::model::task::SharedTask;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

pub type PriorityId = usize;

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

    pub fn note(&mut self, note: String) {
        self.notes.push(note)
    }

    pub fn update_priority(
        &mut self,
        priority_id: PriorityId,
        _is_done: bool,
    ) -> Result<(), Error> {
        match self.priorities.get(priority_id) {
            None => Err(Error::default()),
            Some(_priority) => {
                todo!();
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
