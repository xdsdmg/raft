use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Clock {
    terminate_signal: Arc<AtomicBool>,
    done_sender: Sender<()>,
}

impl Clock {
    pub fn new(terminate_signal: Arc<AtomicBool>, done_sender: Sender<()>) -> Self {
        Clock {
            terminate_signal,
            done_sender,
        }
    }

    pub fn spin(&self) {
        loop {
            if self.terminate_signal.load(Ordering::SeqCst) {
                println!("info: start terminating crontab service");
                break;
            }
            println!("crontab service is triggered");
            thread::sleep(Duration::from_millis(5000));
        }
        println!("info: crontab service has been terminated");
    }

    pub fn done(&self) {
        let _ = self.done_sender.send(());
    }
}
