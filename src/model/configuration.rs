use crate::cmd_line;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub conf_path: Option<String>,
    pub server_id: Option<usize>,
    pub listen_address: Option<String>,
    pub nodes: Vec<String>,
    pub election_timeout: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            conf_path: None,
            server_id: None,
            listen_address: None,
            nodes: Vec::new(),
            election_timeout: 150, // Unit in milliseconds
        }
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

pub fn get_configuration(args: Vec<String>) -> Configuration {
    let mut cfg = match cmd_line::parse_cmd_line_arg(args) {
        Ok(cfg) => cfg,
        Err(msg) => panic!("{}", msg),
    };

    if let Some(conf_path) = cfg.conf_path {
        let mut cfg_ = read_configuration(&conf_path);
        cfg_.conf_path = Some(conf_path);
        cfg = cfg_;
    }

    cfg
}

fn read_configuration(path: &str) -> Configuration {
    let context = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            panic!("error: read {} failed, {}", path, e)
        }
    };

    let cfg: Configuration = match serde_json::from_str(&context) {
        Ok(c) => c,
        Err(e) => {
            panic!("error: deserialize configuration failed, {}", e)
        }
    };

    cfg
}
