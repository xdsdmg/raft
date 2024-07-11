use std::{
    sync::{atomic::AtomicBool, mpsc::Sender, Arc},
    thread,
};

use crate::{crontab::Clock, model::configuration::Configuration, rpc::RPC};

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
}

impl Server {
    pub fn new(
        cfg: &Configuration,
        terminate_signal: Arc<AtomicBool>,
        done_sender: Sender<()>,
    ) -> Self {
        let host = cfg
            .listen_address
            .as_ref()
            .expect("error: listen address is empty");

        let rpc_service = RPC::new(&host, terminate_signal.clone(), done_sender.clone());
        let clock_service = Clock::new(terminate_signal.clone(), done_sender.clone());

        Server {
            role: Role::Follower,
            term: 0,
            rpc_service: Arc::new(rpc_service),
            clock_service: Arc::new(clock_service),
        }
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
}
