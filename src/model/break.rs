use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Deserialize, Serialize)]
pub struct Break {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

impl Break {
    pub fn new(start: NaiveDateTime) -> Self {
        Self { start, end: None }
    }

    pub fn end(&mut self) -> Result<(), BreakError> {
        match &mut self.end {
            Some(_) => {
                self.end = Some(Local::now().naive_local());
                Ok(())
            }
            None => Err(BreakError::BreakAlreadyEnded),
        }
    }
}

#[derive(Error, Debug)]
pub enum BreakError {
    #[error("No break started")]
    NoBreak,
    #[error("Break already ended")]
    BreakAlreadyEnded,
    #[error("Last break is still going")]
    LastBreakNotEnded,
    #[error("No focus exists")]
    NoFocus,
}
