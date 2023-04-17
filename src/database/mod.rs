pub mod context;
mod data;

use crate::database::data::Data;
use crate::model::day::Day;
use chrono::NaiveDate;
use context::Context;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::{fs, io};

pub static DATABASE: LazyLock<Mutex<Database>> = LazyLock::new(|| {
    let mut location = dirs::config_dir().expect("OS does not have a config directory");
    location.push("doer");
    if !location.exists() {
        fs::create_dir(&location).unwrap();
    }
    Mutex::new(Database::new(location).expect("Database creation failed"))
});

pub struct Database {
    data: Data,
    file_path: PathBuf,
}

impl Database {
    pub fn new(mut location: PathBuf) -> Result<Self, Error> {
        location.push("database.json");

        let mut database = Self {
            data: Data::default(),
            file_path: location,
        };
        database.load()?;
        Ok(database)
    }

    pub fn context(&self) -> &String {
        &self.data.context
    }

    pub fn set_context(&mut self, context: String) {
        if !self.data.contexts.contains_key(&context) {
            self.data
                .contexts
                .insert(context.clone(), Context::default());
        }

        self.data.context = context;
    }

    pub fn contexts(&self) -> Vec<&String> {
        self.data.contexts.keys().collect()
    }

    pub fn get(&self, date: &NaiveDate) -> Option<&Day> {
        self.data
            .contexts
            .get(&self.data.context)
            .unwrap()
            .get(date)
    }

    pub fn set(&mut self, date: NaiveDate, day: Day) {
        self.data
            .contexts
            .get_mut(&self.data.context)
            .unwrap()
            .insert(date, day);
    }

    fn load(&mut self) -> Result<(), Error> {
        if !self.file_path.exists() {
            return Ok(());
        }

        let mut file = File::open(&self.file_path)?;
        let mut db_str = String::new();
        file.read_to_string(&mut db_str)?;
        self.data = serde_json::from_str(&db_str)?;

        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        let db_str = serde_json::to_string_pretty(&self.data)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(db_str.as_ref())?;
        Ok(())
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.save().expect("Database failed to save");
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serialise(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => Display::fmt(&err, f),
            Error::Serialise(err) => Display::fmt(&err, f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialise(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};

    fn temp_database() -> (Database, TempDir) {
        let dir = tempdir().unwrap();
        (Database::new(dir.path().to_path_buf()).unwrap(), dir)
    }

    #[test]
    fn context() {
        let (database, _dir) = temp_database();
        assert_eq!(database.context(), "default");
        drop(database)
    }

    #[test]
    fn set_context() {
        let (mut database, _dir) = temp_database();
        database.set_context("work".to_string());
        assert_eq!(database.context(), "work");
        drop(database)
    }

    #[test]
    fn data_persists() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().to_path_buf();

        {
            let mut database = Database::new(dir_path.clone()).unwrap();
            assert_eq!(database.context(), "default");
            database.set_context("play".to_string());
        }

        {
            let database = Database::new(dir_path).unwrap();
            assert_eq!(database.context(), "play");
        }
    }

    #[test]
    fn get_missing_day() {
        let (database, _dir) = temp_database();
        let day = database.get(&NaiveDate::default());
        assert!(day.is_none());
        drop(database)
    }

    #[test]
    fn set_day() {
        let (mut database, _dir) = temp_database();
        let note = "A note";
        let date = NaiveDate::default();

        let mut day = Day::default();
        day.note(note.to_string());

        database.set(date.clone(), day.clone());
        let day2 = database.get(&date).unwrap();

        assert_eq!(day, *day2);

        drop(database)
    }
}
