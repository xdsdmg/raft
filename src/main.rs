mod cmd_line;
mod crontab;
mod error;
mod model;
mod rpc;
mod server;
mod signal;

use std::{
    env,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
};

use model::configuration::{self, Configuration};
use server::Server;

fn wait(wait_count: Arc<AtomicU32>, rx: Receiver<()>) {
    for _ in rx {
        wait_count.fetch_sub(1, Ordering::SeqCst);
        if wait_count.load(Ordering::SeqCst) == 0 {
            break;
        }
    }

    println!("info: all background threads have been terminated");
}

fn start(cfg: &Configuration) {
    let server = Server::new(cfg, terminate_signal.clone(), tx.clone());
    server.run();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = configuration::get_configuration(args);
    println!("cfg: {}", cfg);
    start(&cfg);
}
