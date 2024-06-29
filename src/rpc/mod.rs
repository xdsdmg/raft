mod tests;

use std::{
    io::{ErrorKind, Read},
    net::{TcpListener, TcpStream},
    sync::mpsc::{Receiver, TryRecvError},
};

pub struct RPC {
    host: String,
    rx: Receiver<()>,
}

impl RPC {
    pub fn new(host: &str, rx: Receiver<()>) -> Self {
        RPC {
            host: String::from(host),
            rx,
        }
    }

    pub fn spin(&self) {
        let listener = TcpListener::bind(&self.host).expect("error: RPC service startup failed");
        let _ = listener.set_nonblocking(true);

        println!("RPC service is running at {}", self.host);

        for stream in listener.incoming() {
            match self.rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("info: start terminating RPC service");
                    break;
                }
                Err(TryRecvError::Empty) => {}
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
