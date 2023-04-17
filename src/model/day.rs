use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Day {
    notes: Vec<String>,
}

impl Day {
    pub fn note(&mut self, note: String) {
        self.notes.push(note)
    }
}
