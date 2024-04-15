use crate::model::data::Data;
use crate::model::day::{Day, Priority};
use crate::model::edit_day_guard::EditDayGuard;
use crate::storage::Storage;
use chrono::{Local, NaiveDate};
use std::io::stdin;

const MAX_NUM_PRIORITIES: usize = 6;

pub type PriorityId = usize;

// TODO: Could use termcolor crate to make the output prettier
fn data() -> Data {
    Data::new(storage().database_dir())
}

fn storage() -> Storage {
    Storage::default()
}

pub fn plan_priorities(date: NaiveDate) {
    {
        let mut data = data();
        let mut edit_guard = EditDayGuard::new(date, &mut data);
        let mut items = vec![];

        if !edit_guard.day().priorities.is_empty() {
            show_priorities(edit_guard.day());
            items.extend(edit_guard.day().priorities.clone())
        }

        items.extend(collect_items());
        edit_guard.day().priorities = order_items(&items);
    }
    show_priorities(&data().day(&date));
}

pub fn copy_priorities(date_from: &NaiveDate, date_to: &NaiveDate) {
    {
        let from = data().day(date_from);
        let mut data = data();
        let mut edit_guard = EditDayGuard::new(*date_to, &mut data);
        edit_guard.day().priorities = from.priorities.clone();
    }
    show_priorities(&data().day(date_to));
}

pub fn show(date: &NaiveDate) {
    show_day(&data().day(date));
}

pub fn show_last() {
    match last_date() {
        None => {
            println!("No data to show")
        }
        Some(date) => {
            println!("Last day was {date}:");
            show_day(&data().day(&date));
        }
    }
}

pub fn last_date() -> Option<NaiveDate> {
    data().last_date()
}

pub fn tick(id: &PriorityId) {
    set_tick(id, true);
}

pub fn un_tick(id: &PriorityId) {
    set_tick(id, false);
}

fn set_tick(id: &PriorityId, state: bool) {
    let mut data = data();
    let mut edit_guard = EditDayGuard::new(today(), &mut data);
    let max_id = edit_guard.day().priorities.len() - 1;
    if id > &max_id {
        println!("Invalid id {id}, maximum is {max_id}");
        return;
    }
    edit_guard.day().priorities[*id].done = state;
}

pub fn context() {
    // TODO: It doesn't make sense that database is an object but context is a global function
    println!("{}", storage().context())
}

pub fn contexts() {
    // TODO: Try the default print of a vector, or implement Display for Contexts
    for context in storage().contexts() {
        println!("{}", context)
    }
}

pub fn set_context(context: String) {
    storage().set_context(context)
}

pub fn set_focus(focus: &str) {
    let mut data = data();
    let mut edit_guard = EditDayGuard::new(today(), &mut data);
    edit_guard.day().set_focus(focus.to_string());
}

pub fn set_focus_to_priority(id: PriorityId) {
    let mut data = data();
    let mut edit_guard = EditDayGuard::new(today(), &mut data);
    let max_id = edit_guard.day().priorities.len() - 1;
    if id > max_id {
        println!("Invalid id {id}, maximum is {max_id}");
        return;
    }
    // TODO: Find a better way of doing this than simply copying the name. The focus and priority should reference the same task object more explicitly.
    let focus_name = edit_guard.day().priorities[id].name.clone();
    edit_guard.day().set_focus(focus_name);
}

pub fn start_break() {
    let mut data = data();
    let mut edit_guard = EditDayGuard::new(today(), &mut data);
    match edit_guard.day().start_break() {
        Ok(_) => {}
        Err(err) => {
            println!("{err}")
        }
    }
}

pub fn end_break() {
    let mut data = data();
    let mut edit_guard = EditDayGuard::new(today(), &mut data);
    match edit_guard.day().end_break() {
        Ok(_) => {}
        Err(err) => {
            println!("{err}")
        }
    }
}

pub fn end_day() {
    let mut data = data();
    let mut edit_guard = EditDayGuard::new(today(), &mut data);
    edit_guard.day().end();
}

pub fn add_note(note: String) {
    let mut data = data();
    let mut edit_guard = EditDayGuard::new(today(), &mut data);
    edit_guard.day().add_note(note);
}

// TODO: Use inquire crate for better user input collecting.
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

// TODO: Put all of this string formatting into impl Display for Day
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
