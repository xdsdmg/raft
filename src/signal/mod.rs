use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

pub fn init(terminate_signal: Arc<AtomicBool>) {
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
