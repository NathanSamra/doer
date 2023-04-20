use crate::model::focus::Error;
use chrono::NaiveTime;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct FocusBreak {
    start: NaiveTime,
    end: Option<NaiveTime>,
}

impl FocusBreak {
    pub fn start() -> Self {
        Self {
            start: chrono::Local::now().naive_local().time(),
            end: None,
        }
    }

    pub fn end(&mut self) -> Result<(), Error> {
        match self.end {
            None => {
                self.end = Some(chrono::Local::now().naive_local().time());
                Ok(())
            }
            Some(_) => Err(Error::NotInBreak),
        }
    }

    pub fn is_over(&self) -> bool {
        self.end.is_some()
    }
}

impl Display for FocusBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.end {
            None => {
                writeln!(f, "{} -", self.start)
            }
            Some(end) => {
                writeln!(f, "{} - {}", self.start, end)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_not_over() {
        let break_ = FocusBreak::start();
        assert!(!break_.is_over());
    }

    #[test]
    fn is_over() {
        let mut break_ = FocusBreak::start();
        break_.end().unwrap();
        assert!(break_.is_over());
    }

    #[test]
    fn ending_twice_fails() {
        let mut break_ = FocusBreak::start();
        break_.end().unwrap();
        let result = break_.end();
        assert!(result.is_err());
    }
}
