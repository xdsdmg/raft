mod tests;
use crate::error::Error;

use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
};

pub struct RPC {
    host: String,
    terminate_signal: Arc<AtomicBool>,
    done_sender: Sender<()>,
}

impl RPC {
    pub fn new(host: &str, terminate_signal: Arc<AtomicBool>, done_sender: Sender<()>) -> Self {
        RPC {
            host: String::from(host),
            terminate_signal,
            done_sender,
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

    pub fn done(&self) {
        let _ = self.done_sender.send(());
    }
    
    // Join the cluster:
    // For finding the leader, The currnet node will attempt to estiblish a TCP connection to each nodes specified in configuration file.

    pub fn join_cluster(&self, msg: &str, addr: &str) -> Result<(), Error> {
        let mut stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(e) => {
                println!("tcp connection failed, error: {}", e);
                return Err(Error::TcpConnectionFailed);
            }
        };

        if let Err(_) = stream.write(msg.as_bytes()) {
            return Err(Error::TcpConnectionFailed);
        }

        Ok(())
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
