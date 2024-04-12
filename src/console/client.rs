use chrono::NaiveDate;

pub type PriorityId = i32;

// TODO: This was designed when I was considering web. Maybe remove the 'client-ness' of it.
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        todo!()
    }

    pub fn plan_priorities(&mut self, _date: &NaiveDate) {
        todo!()
    }

    pub fn copy_priorities(&mut self, _from: &NaiveDate, _to: &NaiveDate) {
        todo!()
    }

    pub fn show(&self, _date: &NaiveDate) {
        todo!()
    }

    pub fn last_date(&self) -> NaiveDate {
        todo!()
    }

    pub fn tick(&mut self, _id: &PriorityId) {
        todo!()
    }

    pub fn un_tick(&mut self, _id: &PriorityId) {
        todo!()
    }

    pub fn context(&self) {
        todo!()
    }

    pub fn contexts(&self) {
        todo!()
    }

    pub fn set_context(&mut self, _context: &str) {
        todo!()
    }

    pub fn set_focus(&mut self, _focus: &str) {
        todo!()
    }

    pub fn set_focus_to_priority(&mut self, _id: PriorityId) {
        todo!()
    }

    pub fn start_break(&mut self) {
        todo!()
    }

    pub fn end_break(&mut self) {
        todo!()
    }

    pub fn end_day(&mut self) {
        todo!()
    }

    pub fn note(&mut self, _note: &str) {
        todo!()
    }
}
