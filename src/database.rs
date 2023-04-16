use serde::{Deserialize, Serialize};
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
    config: Config,
    location: PathBuf,
}

impl Database {
    pub fn new(location: PathBuf) -> Result<Self, Error> {
        let mut database = Self {
            config: Config::default(),
            location,
        };
        database.load()?;
        Ok(database)
    }

    pub fn context(&self) -> &String {
        &self.config.context
    }

    pub fn set_context(&mut self, context: String) {
        self.config.context = context;
    }

    fn config_path(&self) -> PathBuf {
        let mut context_file = self.location.clone();
        context_file.push("config.toml");
        context_file
    }

    fn load(&mut self) -> Result<(), Error> {
        let config_path = self.config_path();
        if !config_path.exists() {
            return Ok(());
        }

        let mut config_file = File::open(config_path)?;
        let mut config_str = String::new();
        config_file.read_to_string(&mut config_str)?;
        self.config = toml::from_str(&config_str)?;
        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        let config_str = toml::to_string_pretty(&self.config)?;
        let mut config_file = File::create(self.config_path())?;
        config_file.write_all(config_str.as_ref())?;
        Ok(())
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.save().expect("Database failed to save");
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    pub context: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            context: "default".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Deserialize(toml::de::Error),
    Serialise(toml::ser::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => Display::fmt(&err, f),
            Error::Deserialize(err) => Display::fmt(&err, f),
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

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::Deserialize(value)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::Serialise(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    fn temp_database() -> Database {
        let dir = temp_dir();
        Database::new(dir).unwrap()
    }

    #[test]
    fn context() {
        let database = temp_database();
        assert_eq!(database.context(), "default");
    }

    #[test]
    fn set_context() {
        let mut database = temp_database();
        database.set_context("work".to_string());
        assert_eq!(database.context(), "work");
    }

    #[test]
    fn config_persists() {
        let dir = temp_dir();

        {
            let mut database = Database::new(dir.clone()).unwrap();
            assert_eq!(database.context(), "default");
            database.set_context("play".to_string());
        }

        {
            let database = Database::new(dir).unwrap();
            assert_eq!(database.context(), "play");
        }
    }
}
