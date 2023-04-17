use crate::database::context::Context;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub context: String,
    pub contexts: HashMap<String, Context>,
    version: Version,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            context: "default".to_string(),
            contexts: HashMap::from([("default".to_string(), Context::default())]),
            version: Version::parse(env!("CARGO_PKG_VERSION")).unwrap(),
        }
    }
}
