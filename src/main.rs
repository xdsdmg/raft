mod model;
mod rpc;

use core::panic;
use model::configuration::Configuration;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::thread::JoinHandle;
use std::{env, thread};

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

fn init_signal_handler(rpc_done_tx: Sender<()>) {
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGINT => {
                    println!("Received SIGINT, shutting down gracefully...");
                    r.store(false, Ordering::SeqCst);
                    let _ = rpc_done_tx.send(()); // Terminate RPC service
                    break;
                }
                _ => unreachable!(),
            }
        }
    });
}

fn init_rpc_serivce(rpc_done_rx: Receiver<()>) -> JoinHandle<()> {
    thread::spawn(move || {
        let rpc_srv = rpc::RPC::new("127.0.0.1:3456", rpc_done_rx);
        rpc_srv.spin();
    })
}

fn init() {
    let (tx, rx) = mpsc::channel::<()>();
    init_signal_handler(tx);
    let rpc_handle = init_rpc_serivce(rx);
    rpc_handle.join().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = match parse_cmd_line_arg(args) {
        Ok(cfg) => cfg,
        Err(msg) => panic!("{}", msg),
    };

    println!("cfg: {}", cfg);

    init();
}
