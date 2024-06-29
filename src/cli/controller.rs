use crate::database::Database;
use crate::model::task::{Task, TaskId};
use crate::storage::storage_handler::{default_dirs, remove_lock, StorageHandler};
use chrono::{Local, NaiveDate};
use std::io::stdin;

const MAX_NUM_PRIORITIES: usize = 6;

pub type PriorityId = usize;

// TODO: Could use termcolor crate to make the output prettier
pub struct Controller {
    database: Database,
}

impl Controller {
    pub fn new(storage: StorageHandler) -> Controller {
        Controller {
            database: Database::load(storage),
        }
    }

    pub fn add_task(&mut self, date: NaiveDate, name: String) {
        let task = Task::new(name);
        let mut day = self.database.get_day(&date).clone();
        // TODO: Handle errors
        if day.insert_task(task).is_some() {
            panic!("New task already exists in the day");
        }
        self.database.set_day(date, day);
    }

    pub fn plan_priorities(&mut self, date: NaiveDate) {
        let mut day = self.database.get_day(&date).clone();
        let tasks = day.tasks();
        let priorities = order_tasks(&tasks);
        day.set_priorities(priorities);
        self.database.set_day(date, day);
        println!("Planning complete")
    }

    pub fn copy_tasks(
        &mut self,
        date_from: &NaiveDate,
        date_to: &NaiveDate,
        include_finished: bool,
    ) {
        let binding = self.database.get_day(date_from);
        let mut tasks = binding.tasks().clone();
        if !include_finished {
            tasks.retain(|task| !task.done);
        }
        let mut day = self.database.get_day(date_to).clone();
        for task in tasks {
            day.insert_task(task.clone());
        }
        self.database.set_day(*date_to, day);
    }

    pub fn show(&self, date: &NaiveDate) {
        let day = self.database.get_day(date);
        println!("{}", day);
    }

    pub fn show_last(&self) {
        match self.last_date() {
            None => {
                println!("No data to show")
            }
            Some(date) => {
                println!("Last day was {date}:");
                self.show(&date);
            }
        }
    }

    pub fn last_date(&self) -> Option<NaiveDate> {
        self.database.last_date()
    }

    pub fn tick(&mut self, date: &NaiveDate, id: PriorityId) {
        self.set_tick(date, id, true);
    }

    pub fn un_tick(&mut self, date: &NaiveDate, id: PriorityId) {
        self.set_tick(date, id, false);
    }

    fn set_tick(&mut self, date: &NaiveDate, id: PriorityId, state: bool) {
        let mut day = self.database.get_day(date);
        let max_id = day.priorities().len() - 1;
        if id > max_id {
            println!("Invalid id {id}, maximum is {max_id}");
            return;
        }
        let task_id = day.priorities()[id - 1];
        // Handle errors
        let mut task = day.get_task(&task_id).unwrap().clone();
        task.done = state;
        day.insert_task(task);
        self.database.set_day(*date, day);
    }

    pub fn context(&self) {
        println!("{}", self.database.context())
    }

    pub fn contexts(&self) {
        // TODO: Try the default print of a vector, or implement Display for Contexts
        for context in self.database.contexts() {
            println!("{}", context)
        }
    }

    pub fn set_context(&mut self, context: String) {
        self.database.set_context(context)
    }

    pub fn new_context(&mut self, context: String) {
        self.database.new_context(context)
    }

    // TODO: PriorityId is no longer an appropriate type name
    pub fn start_focus(&mut self, id: PriorityId) {
        let date = today();
        let mut day = self.database.get_day(&date).clone();
        let tasks = day.tasks();
        let task = tasks[id - 1];
        day.start_focus(task.id).unwrap();
        self.database.set_day(date, day);
    }

    pub fn start_focus_on_new_task(&mut self, focus: String) {
        let task = Task::new(focus);
        let task_id = task.id;
        let date = today();
        let mut day = self.database.get_day(&date);
        // Handle errors
        if day.insert_task(task).is_some() {
            panic!("New task already exists in the day");
        }
        day.start_focus(task_id).unwrap();
    }

    pub fn start_break(&mut self) {
        let date = today();
        let mut day = self.database.get_day(&date).clone();
        day.start_break().unwrap();
        self.database.set_day(date, day);
    }

    pub fn start_day(&mut self) {
        let date = today();
        let mut day = self.database.get_day(&date).clone();
        day.start_day().unwrap();
        self.database.set_day(date, day);
    }

    pub fn end_day(&mut self) {
        let date = today();
        let mut day = self.database.get_day(&date).clone();
        day.end_day().unwrap();
        self.database.set_day(date, day);
    }

    pub fn add_note(&mut self, date: &NaiveDate, note: String) {
        let mut day = self.database.get_day(date).clone();
        day.add_note(note);
        self.database.set_day(*date, day);
    }

    pub fn remove_lock(&mut self) {
        println!("Warning: Ensure no other instances are running");
        // TODO: Handle errors
        remove_lock(&default_dirs()).unwrap();
    }
}

// TODO: Use inquire crate for better user input collecting.
// TODO: Have the unfinished items from the previous day (handle weekends? Last day with items?) be added automatically
fn order_tasks(items: &Vec<&Task>) -> Vec<TaskId> {
    let mut result = vec![];
    let mut remaining = items.to_owned();

    for _priority in 0..MAX_NUM_PRIORITIES {
        let max_id = remaining.len();
        if max_id == 0 {
            return result;
        }

        println!("Remaining:");
        for (i, task) in remaining.iter().enumerate() {
            let index = i + 1;
            let name = &task.name;
            println!("{index}. {name}");
        }

        match get_item_id(max_id) {
            None => {
                return result;
            }
            Some(choice) => {
                result.push(remaining.remove(choice - 1).id);
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
