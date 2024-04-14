use crate::config::{context, contexts, set_context};
use crate::model::data::Data;
use crate::model::day::{Day, Priority};
use crate::model::edit_day_guard::EditDayGuard;
use crate::storage::database;
use chrono::{Local, NaiveDate};
use std::io::stdin;

const MAX_NUM_PRIORITIES: usize = 6;

pub type PriorityId = usize;

// TODO: This was designed when I was considering web. Maybe remove the 'client-ness' of it.
pub struct Client {
    data: Data,
}

impl Client {
    pub fn new() -> Self {
        let data = Data::new(database(), context());
        Self { data }
    }

    pub fn plan_priorities(&mut self, date: NaiveDate) {
        {
            let mut edit_guard = self.build_edit_guard(date);
            let mut items = vec![];

            if !edit_guard.day().priorities.is_empty() {
                show_priorities(edit_guard.day());
                items.extend(edit_guard.day().priorities.clone())
            }

            items.extend(collect_items());
            edit_guard.day().priorities = order_items(&items);
        }
        show_priorities(&self.data.day(&date));
    }

    pub fn copy_priorities(&mut self, date_from: &NaiveDate, date_to: &NaiveDate) {
        {
            let from = self.data.day(date_from);
            let mut edit_guard = self.build_edit_guard(*date_to);
            edit_guard.day().priorities = from.priorities.clone();
        }
        show_priorities(&self.data.day(date_to));
    }

    pub fn show(&self, date: &NaiveDate) {
        show_day(&self.data.day(date));
    }

    pub fn show_last(&self) {
        match self.last_date() {
            None => {
                println!("No data to show")
            }
            Some(date) => {
                println!("Last day was {date}:");
                show_day(&self.data.day(&date));
            }
        }
    }

    pub fn last_date(&self) -> Option<NaiveDate> {
        self.data.last_date()
    }

    pub fn tick(&mut self, id: &PriorityId) {
        self.set_tick(id, true);
    }

    pub fn un_tick(&mut self, id: &PriorityId) {
        self.set_tick(id, false);
    }

    fn set_tick(&mut self, id: &PriorityId, state: bool) {
        let mut edit_guard = self.build_edit_guard(today());
        let max_id = edit_guard.day().priorities.len() - 1;
        if id > &max_id {
            println!("Invalid id {id}, maximum is {max_id}");
            return;
        }
        edit_guard.day().priorities[*id].done = state;
    }

    pub fn context(&self) {
        // TODO: It doesn't make sense that database is an object but context is a global function
        println!("{}", context())
    }

    pub fn contexts(&self) {
        for context in contexts() {
            println!("{}", context)
        }
    }

    pub fn set_context(&mut self, context: &str) {
        set_context(context)
    }

    pub fn set_focus(&mut self, focus: &str) {
        let mut edit_guard = self.build_edit_guard(today());
        edit_guard.day().set_focus(focus.to_string());
    }

    pub fn set_focus_to_priority(&mut self, id: PriorityId) {
        let mut edit_guard = self.build_edit_guard(today());
        let max_id = edit_guard.day().priorities.len() - 1;
        if id > max_id {
            println!("Invalid id {id}, maximum is {max_id}");
            return;
        }
        // TODO: Find a better way of doing this than simply copying the name. The focus and priority should reference the same task object more explicitly.
        let focus_name = edit_guard.day().priorities[id].name.clone();
        edit_guard.day().set_focus(focus_name);
    }

    pub fn start_break(&mut self) {
        let mut edit_guard = self.build_edit_guard(today());
        edit_guard.day().start_break();
    }

    pub fn end_break(&mut self) {
        let mut edit_guard = self.build_edit_guard(today());
        edit_guard.day().end_break();
    }

    pub fn end_day(&mut self) {
        let mut edit_guard = self.build_edit_guard(today());
        edit_guard.day().end_day();
    }

    pub fn note(&mut self, note: String) {
        let mut edit_guard = self.build_edit_guard(today());
        edit_guard.day().add_note(note);
    }

    fn build_edit_guard(&mut self, date: NaiveDate) -> EditDayGuard {
        EditDayGuard::new(date, &mut self.data)
    }
}

// TODO: Have the unfinished items from the previous day (handle weekends? Last day with items?) be added automatically
fn collect_items() -> Vec<Priority> {
    println!("List items:");
    let mut items = vec![];
    loop {
        let mut line = String::new();
        // TODO: Should handle this potential error
        stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        items.push(Priority::new(line));
        println!("Anymore?")
    }

    items
}

fn order_items(items: &[Priority]) -> Vec<Priority> {
    let mut result = vec![];
    let mut remaining = items.to_owned();

    for _priority in 0..MAX_NUM_PRIORITIES {
        let max_id = remaining.len();
        if max_id == 0 {
            return result;
        }

        println!("Remaining:");
        for (i, item) in remaining.iter().enumerate() {
            let index = i + 1;
            let item_name = &item.name;
            println!("{index}. {item_name}");
        }

        match get_item_id(max_id) {
            None => {
                return result;
            }
            Some(choice) => {
                result.push(remaining.remove(choice - 1));
            }
        }
    }

    result
}

fn get_item_id(max_id: usize) -> Option<usize> {
    loop {
        let mut choice = String::new();
        // TODO: Handle this error
        stdin().read_line(&mut choice).unwrap();
        if choice.is_empty() {
            return None;
        }

        match choice.parse::<usize>() {
            Ok(id) => {
                if id > max_id {
                    println!("{id} is too large, maximum is {max_id}");
                    continue;
                }

                return Some(id);
            }
            Err(_) => {
                println!("{choice} is not a digit, please try again");
                continue;
            }
        }
    }
}

fn today() -> NaiveDate {
    Local::now().date_naive()
}

fn show_day(day: &Day) {
    show_priorities(day);
    show_log(day);
    show_notes(day);
}

fn show_priorities(day: &Day) {
    if day.priorities.is_empty() {
        println!("No priorities");
        return;
    }

    let focus = match day.focus() {
        None => "",
        Some(focus) => focus.name.as_str(),
    };

    println!("Priorities:");
    for (i, priority) in day.priorities.iter().enumerate() {
        let priority_name = priority.name.as_str();
        let mut line = format!("{i}. {priority_name}");

        if focus == priority_name {
            line += "*";
        }

        if priority.done {
            line += " - done";
        }

        println!("{}", line);
    }

    if !focus.is_empty() {
        println!("\nFocus: {focus}");
    }

    println!();
}

fn show_log(day: &Day) {
    if day.log().is_empty() {
        println!("No log");
        return;
    }

    println!("Log:");
    for focus in day.log() {
        let start = focus.start.format("%H:%M");
        let focus_name = &focus.name;
        // TODO: Instead of putting the formatting in here, why not impl Display for Focus.
        // This could apply to all the other structs as well.
        println!("{start} - {focus_name}");

        for break_ in focus.breaks.iter() {
            let break_start = break_.start.format("%H:%M");

            let break_end = match break_.end {
                None => "N/A".to_string(),
                Some(end) => end.format("%H:%M").to_string(),
            };

            println!("\t{break_start} - {break_end}");
        }
    }

    println!();
}

fn show_notes(day: &Day) {
    if day.notes().is_empty() {
        println!("No notes");
        return;
    }

    println!("Notes:");
    for (i, note) in day.notes().iter().enumerate() {
        println!("{i}. {note}");
    }

    println!();
}
