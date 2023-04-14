use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::default);

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub database: Database,
}

impl Default for Config {
    fn default() -> Self {
        let config_file = config_path();

        let config_str = match File::open(config_file) {
            Ok(mut file) => {
                let mut config_str = String::new();
                file.read_to_string(&mut config_str).unwrap();
                config_str
            }
            Err(_) => "".to_string(),
        };

        let config: Config = toml::from_str(&config_str).unwrap();
        config
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        let config_str = toml::to_string_pretty(&self).unwrap();
        let mut config_file = File::create(config_path()).unwrap();
        config_file.write_all(config_str.as_ref()).unwrap();
    }
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    #[serde(default)]
    pub context: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            context: "default".to_string(),
        }
    }
}

fn config_path() -> PathBuf {
    let mut config_file = dirs::config_dir().expect("OS does not have a config directory");
    config_file.push("doer");
    if !config_file.exists() {
        fs::create_dir(&config_file).unwrap();
    }
    config_file.push("config.toml");
    config_file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write() {
        let config = Config::default();
        drop(config);
    }
}
