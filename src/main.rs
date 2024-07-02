mod cmd_line;
mod crontab;
mod model;
mod rpc;
mod signal;

use core::panic;
use std::{
    env,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
};

fn wait(wait_count: Arc<AtomicU32>, rx: Receiver<()>) {
    for _ in rx {
        wait_count.fetch_sub(1, Ordering::SeqCst);
        if wait_count.load(Ordering::SeqCst) == 0 {
            break;
        }
    }
}

fn start() {
    let terminate_signal = Arc::new(AtomicBool::new(false));
    let wait_count = Arc::new(AtomicU32::new(0));

    signal::init(terminate_signal.clone());

    let (tx, rx) = mpsc::channel::<()>();

    wait_count.fetch_add(1, Ordering::SeqCst);
    let _ = rpc::init(terminate_signal.clone(), tx.clone());
    wait_count.fetch_add(1, Ordering::SeqCst);
    let _ = crontab::init(terminate_signal.clone(), tx.clone());

    wait(wait_count, rx);

    // if let Err(e) = rpc_handle.join() {
    //     println!("error: RPC thread join failed, {:?}", e);
    // }
    // if let Err(e) = crontab_handle.join() {
    //     println!("error: crontab thread join failed, {:?}", e);
    // }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = match cmd_line::parse_cmd_line_arg(args) {
        Ok(cfg) => cfg,
        Err(msg) => panic!("{}", msg),
    };

    println!("cfg: {}", cfg);

    start();
}
