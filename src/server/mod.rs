use crate::{crontab::Clock, model::configuration::Configuration, rpc::RPC, signal};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
};

pub enum Role {
    Follower,
    Candidate,
    Leader,
}

pub struct Server {
    role: Role,
    term: usize,
    rpc_service: Arc<RPC>,
    clock_service: Arc<Clock>,
    nodes: Vec<String>,

    /* Used for thread control */
    terminate_signal: Arc<AtomicBool>,
    done_sender: Sender<()>,
    done_receiver: Receiver<()>,
}

impl Server {
    pub fn new(cfg: &Configuration) -> Self {
        let host = cfg
            .listen_address
            .as_ref()
            .expect("error: listen address is empty");

        let terminate_signal = Arc::new(AtomicBool::new(false));
        let (done_sender, done_receiver) = mpsc::channel::<()>();

        let rpc_service = RPC::new(&host, terminate_signal.clone(), done_sender.clone());
        let clock_service = Clock::new(terminate_signal.clone(), done_sender.clone());

        Server {
            role: Role::Follower,
            term: 0,
            rpc_service: Arc::new(rpc_service),
            clock_service: Arc::new(clock_service),
            nodes: cfg.nodes.clone(),

            terminate_signal,
            done_sender,
            done_receiver,
        }
    }

    pub fn run(&self) {
        signal::init(self.terminate_signal.clone());

        let wait_count = Arc::new(AtomicU32::new(0));

        wait_count.fetch_add(1, Ordering::SeqCst);
        self.start_rpc_service();

        wait_count.fetch_add(1, Ordering::SeqCst);
        self.start_clock_service();

        /* Find the master node and join the cluster */
        for host in &self.nodes {
            println!("host: {}", host);
            let _ = match self.rpc_service.join_cluster("hello world!", &host) {
                Ok(_) => {}
                Err(e) => {
                    println!(
                        "[ERROR] join cluster through node({}) failed, error: {}",
                        host, e
                    )
                }
            };
        }

        self.wait(wait_count);
    }

    pub fn start_rpc_service(&self) {
        let rpc_service = self.rpc_service.clone();

        thread::spawn(move || {
            rpc_service.spin();
            let _ = rpc_service.done();
        });
    }

    pub fn start_clock_service(&self) {
        let clock_service = self.clock_service.clone();

        thread::spawn(move || {
            clock_service.spin();
            let _ = clock_service.done();
        });
    }

    fn wait(&self, wait_count: Arc<AtomicU32>) {
        for _ in &self.done_receiver {
            wait_count.fetch_sub(1, Ordering::SeqCst);
            if wait_count.load(Ordering::SeqCst) == 0 {
                break;
            }
        }

        println!("info: all background threads have been terminated");
    }
}
