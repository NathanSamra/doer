use crate::model::{focus_break::FocusBreak, task::SharedTask};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub enum Error {
    BreakStillOngoing,
    NotInBreak,
}

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}
