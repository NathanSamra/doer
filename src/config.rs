use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Config<T: for<'a> Deserialize<'a> + Serialize> {
    pub data: T,
    file_path: PathBuf,
}

impl<T: for<'a> Deserialize<'a> + Serialize> Config<T> {
    pub fn new(name: &str) -> Self {
        let file_path = config_path(name);

        let config_str = match File::open(&file_path) {
            Ok(mut file) => {
                let mut config_str = String::new();
                file.read_to_string(&mut config_str).unwrap();
                config_str
            }
            Err(_) => "".to_string(),
        };

        let data: T = toml::from_str(&config_str).unwrap();
        Self { data, file_path }
    }
}

impl<T: for<'a> Deserialize<'a> + Serialize> Drop for Config<T> {
    fn drop(&mut self) {
        let config_str = toml::to_string_pretty(&self.data).unwrap();
        let mut config_file = File::create(&self.file_path).unwrap();
        config_file.write_all(config_str.as_ref()).unwrap();
    }
}

fn config_path(name: &str) -> PathBuf {
    let mut config_file = dirs::config_dir().expect("OS does not have a config directory");
    config_file.push("doer");
    if !config_file.exists() {
        fs::create_dir(&config_file).unwrap();
    }
    config_file.push(format!("{}.toml", name));
    config_file
}
