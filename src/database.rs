use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

pub static DATABASE: LazyLock<Database> = LazyLock::new(Database::default);

pub struct Database {
    config: Config<DatabaseConfig>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            config: Config::<DatabaseConfig>::new("database_config"),
        }
    }
}

impl Database {
    pub fn context(&self) -> &String {
        &self.config.data.context
    }
}

#[derive(Deserialize, Serialize)]
struct DatabaseConfig {
    #[serde(default)]
    pub context: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            context: "default".to_string(),
        }
    }
}
