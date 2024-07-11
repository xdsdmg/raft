mod cmd_line;
mod crontab;
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
    let terminate_signal = Arc::new(AtomicBool::new(false));

    signal::init(terminate_signal.clone());

    let (tx, rx) = mpsc::channel::<()>();

    let server = Server::new(cfg, terminate_signal.clone(), tx.clone());

    let wait_count = Arc::new(AtomicU32::new(0));

    wait_count.fetch_add(1, Ordering::SeqCst);
    server.start_rpc_service();

    wait_count.fetch_add(1, Ordering::SeqCst);
    server.start_clock_service();

    wait(wait_count, rx);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = configuration::get_configuration(args);

    println!("cfg: {}", cfg);

    start(&cfg);
}
