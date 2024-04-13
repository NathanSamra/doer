use chrono::{Local, NaiveDateTime};

pub struct Day {
    pub priorities: Vec<Priority>,
}

impl Day {
    pub fn focus(&self) -> Option<&Focus> {
        todo!()
    }

    pub fn set_focus(&mut self, _focus: String) {
        todo!()
    }

    pub fn start_break(&mut self) {
        todo!()
    }

    pub fn end_break(&mut self) {
        todo!()
    }

    pub fn log(&self) -> &Vec<Focus> {
        todo!()
    }

    pub fn end_day(&mut self) {
        todo!()
    }

    pub fn notes(&self) -> &Vec<String> {
        todo!()
    }

    pub fn add_note(&mut self, _note: String) {
        todo!()
    }
}

#[derive(Clone)]
pub struct Priority {
    pub name: String,
    pub done: bool,
}

impl Priority {
    pub fn new(name: String) -> Self {
        Self { name, done: false }
    }
}

// TODO: Should probably log the end time here also
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
}

pub struct Break {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}
