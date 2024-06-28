use std::net::{TcpListener, TcpStream};

pub struct RPC {
    host: String,
}

impl RPC {
    pub fn new(host: &str) -> Self {
        RPC {
            host: String::from(host),
        }
    }

    pub fn spin(&self) {
        let listener = TcpListener::bind(&self.host).expect("error: RPC service startup failed");
    }
}
