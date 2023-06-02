use crate::model::task::{make_shared_task, Task};
use crate::model::{focus_break::FocusBreak, task::SharedTask};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Focus {
    task: SharedTask,
    breaks: Vec<FocusBreak>,
}

impl Focus {
    pub fn new(task: SharedTask) -> Self {
        Self {
            task,
            breaks: vec![],
        }
    }

    pub fn task(&self) -> &SharedTask {
        &self.task
    }

    pub fn breaks(&self) -> &Vec<FocusBreak> {
        &self.breaks
    }

    pub fn set_breaks(&mut self, breaks: Vec<FocusBreak>) {
        self.breaks = breaks;
    }

    pub fn start_break(&mut self) -> Result<(), Error> {
        match self.breaks.last() {
            None => Ok(()),
            Some(last_break) => {
                if !last_break.is_over() {
                    Err(Error::BreakStillOngoing)
                } else {
                    Ok(())
                }
            }
        }?;

        self.breaks.push(FocusBreak::start());
        Ok(())
    }

    pub fn end_break(&mut self) -> Result<(), Error> {
        match self.breaks.last_mut() {
            None => Err(Error::NotInBreak),
            Some(last_break) => last_break.end(),
        }
    }
}

impl Display for Focus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let task = self.task.borrow();
        writeln!(f, "{}", task)
    }
}

impl From<String> for Focus {
    fn from(value: String) -> Self {
        let task = make_shared_task(Task::new(value));
        Focus::new(task)
    }
}

#[derive(Debug)]
pub enum Error {
    BreakStillOngoing,
    NotInBreak,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BreakStillOngoing => write!(f, "Break in progress"),
            Error::NotInBreak => write!(f, "Not in a break"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::task::{make_shared_task, Task};

    fn new_focus() -> Focus {
        Focus::new(make_shared_task(Task::new("A Task".to_string())))
    }

    #[test]
    fn start_break() {
        let mut focus = new_focus();
        let result = focus.start_break();
        assert!(result.is_ok());
    }

    #[test]
    fn end_break() {
        let mut focus = new_focus();
        focus.start_break().unwrap();
        let result = focus.end_break();
        assert!(result.is_ok());
    }

    #[test]
    fn end_break_without_start_fails() {
        let mut focus = new_focus();
        let result = focus.end_break();
        assert!(result.is_err());
    }
}
