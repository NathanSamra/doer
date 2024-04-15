use crate::model::r#break::{Break, BreakError};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

// TODO: Should probably log the end time here also
#[derive(Clone, Deserialize, Serialize)]
pub struct Focus {
    pub name: String,
    pub start: NaiveDateTime,
    pub breaks: Vec<Break>,
}

impl Focus {
    pub fn now(name: String) -> Self {
        Self {
            name,
            start: Local::now().naive_local(),
            breaks: vec![],
        }
    }

    pub fn start_break(&mut self) -> Result<(), BreakError> {
        match self.breaks.last() {
            None => {}
            Some(last_break) => {
                if last_break.end.is_none() {
                    return Err(BreakError::LastBreakNotEnded);
                }
            }
        };

        self.breaks.push(Break::new(Local::now().naive_local()));
        Ok(())
    }

    pub fn end_break(&mut self) -> Result<(), BreakError> {
        match self.breaks.last_mut() {
            Some(break_) => break_.end(),
            None => Err(BreakError::NoBreak),
        }
    }
}
