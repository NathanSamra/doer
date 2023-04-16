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
        let config_str = match File::open(self.config_path()) {
            Ok(mut file) => {
                let mut config_str = String::new();
                file.read_to_string(&mut config_str)?;
                config_str
            }
            Err(_) => "".to_string(),
        };

        self.config = toml::from_str(&config_str)?;
        Ok(())
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let config_str = toml::to_string_pretty(&self.config).expect("Failed to serialise data");
        let mut config_file =
            File::create(self.config_path()).expect("Failed to create config file");
        config_file
            .write_all(config_str.as_ref())
            .expect("Failed to write to config file");
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    #[serde(default)]
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
    Toml(toml::de::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => Display::fmt(&err, f),
            Error::Toml(err) => Display::fmt(&err, f),
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
        Self::Toml(value)
    }
}
