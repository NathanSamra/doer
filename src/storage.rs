use crate::metadata::app_version;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

const DEFAULT_CONTEXT: &str = "default";

// TODO: Move to own file
pub struct Storage {
    dirs: ProjectDirs,
}

// TODO: Use a completely temp dir for testing
impl Storage {
    pub fn database_dir(&self) -> PathBuf {
        // Handle errors
        let context = self.context();
        let dir = self.root().join(context);
        if !dir.exists() {
            fs::create_dir_all(&dir).unwrap();
        }
        dir
    }

    pub fn context(&self) -> String {
        self.state().context.current
    }

    pub fn contexts(&self) -> Vec<String> {
        // TODO: Handle errors
        let contexts: Vec<String> = WalkDir::new(self.root())
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                if entry.file_type().is_dir() {
                    Some(entry.file_name().to_str().unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        if contexts.is_empty() {
            vec![DEFAULT_CONTEXT.to_string()]
        } else {
            contexts
        }
    }

    pub fn set_context(&self, context: String) {
        let mut state = self.state();
        state.context.current = context;
        self.set_state(state);
    }

    fn root(&self) -> PathBuf {
        let version_major = app_version().major;
        self.dirs.data_dir().join(format!("v{version_major}"))
    }

    fn state(&self) -> State {
        // TODO: Handle errors
        let file = self.root().join("state.toml");
        if file.exists() {
            let mut state_str = String::new();
            File::open(self.state_file())
                .unwrap()
                .read_to_string(&mut state_str)
                .unwrap();
            toml::from_str(state_str.as_str()).unwrap()
        } else {
            State::default()
        }
    }

    fn set_state(&self, state: State) {
        // TODO: Handle errors
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.state_file())
            .unwrap();
        let state_str = toml::to_string(&state).unwrap();
        f.write_all(state_str.as_bytes()).unwrap();
    }

    fn state_file(&self) -> PathBuf {
        self.root().join("state.toml")
    }
}

impl Default for Storage {
    fn default() -> Self {
        // TODO: Add qualifier?
        // TODO: Add organisation?
        // TODO: Handle errors
        let dirs = ProjectDirs::from("", "", "doer").unwrap();
        Self { dirs }
    }
}

#[derive(Deserialize, Serialize)]
struct Context {
    pub current: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            current: DEFAULT_CONTEXT.to_string(),
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
struct State {
    pub context: Context,
}
