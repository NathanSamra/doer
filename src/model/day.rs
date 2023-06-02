use crate::model::focus::Focus;
use crate::model::focus_break::FocusBreak;
use crate::model::task::{SharedTask, Task};
use serde::de::MapAccess;
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

pub type PriorityId = usize;

const MAX_PRIORITIES: usize = 6;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Day {
    priorities: Vec<SharedTask>,
    focuses: Vec<Focus>,
    notes: Vec<String>,
}

impl Day {
    pub fn focus(&self) -> Option<&Focus> {
        self.focuses.last()
    }

    pub fn focus_mut(&mut self) -> Option<&mut Focus> {
        self.focuses.last_mut()
    }

    pub fn set_focus(&mut self, focus: Focus) {
        self.focuses.push(focus);
    }

    pub fn set_focus_from_priority(&mut self, priority_id: &PriorityId) -> Result<(), Error> {
        let task = match self.priorities.get(*priority_id) {
            None => Err(Error::InvalidPriorityId),
            Some(priority) => Ok(priority.clone()),
        }?;

        let focus = Focus::new(task);
        self.focuses.push(focus);
        Ok(())
    }

    pub fn priority_mut(&mut self, priority_id: PriorityId) -> Option<RefMut<Task>> {
        self.priorities.get(priority_id).map(|m| m.borrow_mut())
    }

    pub fn priorities(&self) -> Vec<Task> {
        self.priorities
            .iter()
            .map(|p| p.borrow().to_owned())
            .collect()
    }

    pub fn set_priorities(&mut self, priorities: Vec<Task>) -> Result<(), Error> {
        if priorities.len() > MAX_PRIORITIES {
            return Err(Error::TooManyPriorities);
        }

        self.priorities = priorities
            .into_iter()
            .map(|p| Rc::new(RefCell::new(p)))
            .collect();
        Ok(())
    }

    pub fn note(&mut self, note: String) {
        self.notes.push(note)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.notes.is_empty() {
            writeln!(f, "No notes")?
        }

        writeln!(f, "Notes:")?;
        for (i, note) in self.notes.iter().enumerate() {
            writeln!(f, "{}. {}", i, note)?
        }

        Ok(())
    }
}

const DAY_TAG: &str = "day";
const TASKS_TAG: &str = "tasks";
const PRIORITIES_TAG: &str = "priorities";
const FOCUSES_TAG: &str = "focuses";
const NOTES_TAG: &str = "notes";

impl Serialize for Day {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut struct_ser = serializer.serialize_struct(DAY_TAG, 4)?;

        let mut tasks: Vec<Task> = self
            .priorities
            .iter()
            .map(|priority| priority.borrow().deref().clone())
            .collect();

        for focus in &self.focuses {
            let task = focus.task().borrow().deref().clone();
            if !tasks.contains(&task) {
                tasks.push(task);
            }
        }

        struct_ser.serialize_field(TASKS_TAG, &tasks)?;

        let priorities: Vec<usize> = self
            .priorities
            .iter()
            .map(|priority| {
                tasks
                    .iter()
                    .position(|task| task == priority.borrow().deref())
                    .unwrap()
            })
            .collect();

        struct_ser.serialize_field(PRIORITIES_TAG, &priorities)?;

        let focuses: HashMap<usize, Vec<FocusBreak>> = self
            .focuses
            .iter()
            .map(|focus| {
                let index = tasks
                    .iter()
                    .position(|task| task == focus.task().borrow().deref())
                    .unwrap();
                (index, focus.breaks().clone())
            })
            .collect();

        struct_ser.serialize_field(FOCUSES_TAG, &focuses)?;
        struct_ser.serialize_field(NOTES_TAG, &self.notes)?;
        struct_ser.end()
    }
}

impl<'de> Deserialize<'de> for Day {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            DAY_TAG,
            &[TASKS_TAG, PRIORITIES_TAG, FOCUSES_TAG, NOTES_TAG],
            DeDayVisitor,
        )
    }
}

struct DeDayVisitor;

impl<'de> de::Visitor<'de> for DeDayVisitor {
    type Value = Day;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("struct Day")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut maybe_tasks: Option<Vec<Task>> = None;
        let mut maybe_priorities_ids: Option<Vec<usize>> = None;
        let mut maybe_focuses_ids: Option<HashMap<usize, Vec<FocusBreak>>> = None;
        let mut maybe_notes: Option<Vec<String>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                TASKS_TAG => maybe_tasks = Some(map.next_value()?),
                PRIORITIES_TAG => maybe_priorities_ids = Some(map.next_value()?),
                FOCUSES_TAG => maybe_focuses_ids = Some(map.next_value()?),
                NOTES_TAG => maybe_notes = Some(map.next_value()?),
                &_ => panic!(),
            }
        }

        let tasks = maybe_tasks.ok_or(de::Error::missing_field(TASKS_TAG))?;
        let priorities_ids =
            maybe_priorities_ids.ok_or(de::Error::missing_field(PRIORITIES_TAG))?;
        let focuses_ids = maybe_focuses_ids.ok_or(de::Error::missing_field(FOCUSES_TAG))?;
        let notes = maybe_notes.ok_or(de::Error::missing_field(NOTES_TAG))?;

        let shared_tasks: Vec<SharedTask> = tasks
            .into_iter()
            .map(|task| Rc::new(RefCell::new(task)))
            .collect();

        let priorities: Vec<SharedTask> = priorities_ids
            .iter()
            .map(|i| shared_tasks[*i].clone())
            .collect();

        let focuses: Vec<Focus> = focuses_ids
            .into_iter()
            .map(|(i, breaks)| {
                let mut focus = Focus::new(shared_tasks[i].clone());
                focus.set_breaks(breaks);
                focus
            })
            .collect();

        let day = Day {
            priorities,
            focuses,
            notes,
        };

        Ok(day)
    }
}

#[derive(Debug)]
pub enum Error {
    TooManyPriorities,
    InvalidPriorityId,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TooManyPriorities => {
                write!(f, "Too many priorities, maximum is {}", MAX_PRIORITIES)
            }
            Error::InvalidPriorityId => {
                write!(f, "Invalid priority ID")
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_is_none_when_empty() {
        let day = Day::default();
        assert!(day.focus().is_none());
    }

    #[test]
    fn set_one_focus() {
        let mut day = Day::default();
        let focus = Focus::from("A focus".to_string());

        day.set_focus(focus.clone());

        assert_eq!(day.focus().unwrap(), &focus);
    }

    #[test]
    fn set_two_focuses() {
        let mut day = Day::default();
        let focus = Focus::from("A focus".to_string());

        day.set_focus(Focus::from("First focus".to_string()));
        day.set_focus(focus.clone());

        assert_eq!(day.focus().unwrap(), &focus);
    }

    #[test]
    fn set_priority_from_focus() {
        let focus_str = "A focus".to_string();
        let priorities = vec![
            Task::new(focus_str.clone()),
            Task::new("A different task".to_string()),
        ];

        let mut day = Day::default();
        day.set_priorities(priorities).unwrap();
        day.set_focus_from_priority(&0).unwrap();

        assert_eq!(day.focus().unwrap(), &Focus::from(focus_str));
    }

    #[test]
    fn set_priority_from_missing_focus_fails() {
        let focus_str = "A focus".to_string();
        let priorities = vec![
            Task::new(focus_str.clone()),
            Task::new("A different task".to_string()),
        ];

        let mut day = Day::default();
        day.set_priorities(priorities).unwrap();
        let result = day.set_focus_from_priority(&2);

        assert!(result.is_err());
    }

    #[test]
    fn note() {
        let mut day = Day::default();
        day.note("A note".to_string());

        let expected = vec!["A note".to_string()];

        assert_eq!(day.notes, expected)
    }
}
