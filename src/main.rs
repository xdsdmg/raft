mod cmd_line;
mod crontab;
mod model;
mod rpc;
mod signal;

use std::{
    env,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
};

use crate::model::configuration::{self, Configuration};

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
    let wait_count = Arc::new(AtomicU32::new(0));

    signal::init(terminate_signal.clone());

    let (tx, rx) = mpsc::channel::<()>();

    wait_count.fetch_add(1, Ordering::SeqCst);
    let listen_address = cfg
        .listen_address
        .as_ref()
        .expect("error: listen address of RPC is empty");
    let _ = rpc::init(terminate_signal.clone(), tx.clone(), listen_address);

    wait_count.fetch_add(1, Ordering::SeqCst);
    let _ = crontab::init(terminate_signal.clone(), tx.clone());

    wait(wait_count, rx);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = configuration::get_configuration(args);

    println!("cfg: {}", cfg);

    start(&cfg);
}
