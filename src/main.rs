mod model;
mod rpc;

use core::panic;
use model::configuration::Configuration;
use std::env;

fn has_prefix(s: &str, prefix: &str) -> bool {
    if prefix.len() > s.len() {
        return false;
    }

    prefix == &s[0..prefix.len()]
}

fn parse_cmd_line_arg(args: Vec<String>) -> Result<Configuration, String> {
    let mut cfg = Configuration::default();
    let length = args.len();
    let mut i: usize = 1;

    while i < length {
        if !has_prefix(&args[i], "--") {
            return Err(format!("error: unexpected argument '{}' found", args[i]));
        }

        let name = &args[i][2..args[i].len()];

        if name == "conf_path" {
            i += 1;
            if i >= length {
                return Err(format!("error: the value of argument '{}' is empty", name));
            }

            cfg.conf_path = Some(args[i].clone());
        }

        i += 1;
    }

    Ok(cfg)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = match parse_cmd_line_arg(args) {
        Ok(cfg) => cfg,
        Err(msg) => panic!("{}", msg),
    };

    println!("cfg: {}", cfg);
}
