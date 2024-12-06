mod cmd_line;
mod crontab;
mod error;
mod model;
mod rpc;
mod server;
mod signal;

use std::env;

use model::configuration::{self, Configuration};
use server::Server;

fn start(cfg: &Configuration) {
    let server = Server::new(cfg);
    server.run();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = configuration::get_configuration(args);
    println!("cfg: {}", cfg);
    start(&cfg);
}
