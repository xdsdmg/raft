mod cmd_line;
mod model;
mod rpc;

use core::panic;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

fn init_signal_handler(terminate_signal: Arc<AtomicBool>) {
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGINT => {
                    println!("Received SIGINT, shutting down gracefully...");
                    r.store(false, Ordering::SeqCst);
                    terminate_signal.store(true, Ordering::SeqCst);
                    break;
                }
                _ => unreachable!(),
            }
        }
    });
}

fn init_crontab_service(terminate_signal: Arc<AtomicBool>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            if terminate_signal.load(Ordering::SeqCst) {
                println!("info: start terminating crontab service");
                break;
            }
            println!("crontab service is triggered");
            thread::sleep(Duration::from_millis(5000));
        }

        println!("info: crontab service has been terminated");
    })
}

fn init() {
    let terminate_signal = Arc::new(AtomicBool::new(false));

    init_signal_handler(terminate_signal.clone());

    let rpc_handle = rpc::init(terminate_signal.clone());
    let crontab_handle = init_crontab_service(terminate_signal.clone());

    if let Err(e) = rpc_handle.join() {
        println!("error: RPC thread join failed, {:?}", e);
    }

    if let Err(e) = crontab_handle.join() {
        println!("error: crontab thread join failed, {:?}", e);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = match cmd_line::parse_cmd_line_arg(args) {
        Ok(cfg) => cfg,
        Err(msg) => panic!("{}", msg),
    };

    println!("cfg: {}", cfg);

    init();
}
