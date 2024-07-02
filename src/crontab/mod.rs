use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn init(terminate_signal: Arc<AtomicBool>, done_sender: Sender<()>) -> JoinHandle<()> {
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

        let _ = done_sender.send(());
    })
}
