mod tests;

use std::{
    io::{ErrorKind, Read},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

pub fn init(terminate_signal: Arc<AtomicBool>) -> JoinHandle<()> {
    thread::spawn(move || {
        let rpc_srv = RPC::new("127.0.0.1:3456", terminate_signal);
        rpc_srv.spin();
    })
}

pub struct RPC {
    host: String,
    terminate_signal: Arc<AtomicBool>,
}

impl RPC {
    pub fn new(host: &str, terminate_signal: Arc<AtomicBool>) -> Self {
        RPC {
            host: String::from(host),
            terminate_signal,
        }
    }

    pub fn spin(&self) {
        let listener = TcpListener::bind(&self.host).expect("error: RPC service startup failed");
        let _ = listener.set_nonblocking(true);

        println!("RPC service is running at {}", self.host);

        for stream in listener.incoming() {
            if self.terminate_signal.load(Ordering::SeqCst) {
                println!("info: start terminating RPC service");
                break;
            }

            let stream = match stream {
                Ok(stream) => stream,
                Err(e) => {
                    if e.kind() != ErrorKind::WouldBlock {
                        println!("error: get TCP stream failed, {} ({})", e, e.kind());
                    }
                    continue;
                }
            };
            self.parse_connection(stream)
        }

        println!("info: RPC service has been terminated");
    }

    fn parse_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        if let Err(e) = stream.read(&mut buffer) {
            println!("error: TCP stream read failed, {}", e);
        }

        println!(
            "TCP stream content: {}",
            std::str::from_utf8(&buffer).unwrap()
        );
    }
}
