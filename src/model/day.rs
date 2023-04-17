use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Day {
    notes: Vec<String>,
}

impl Day {
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
