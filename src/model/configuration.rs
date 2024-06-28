use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub conf_path: Option<String>,
    pub server_id: Option<usize>,
    pub listen_address: Option<String>,
    pub nodes: Option<Vec<String>>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            conf_path: None,
            server_id: None,
            listen_address: None,
            nodes: None,
        }
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}
