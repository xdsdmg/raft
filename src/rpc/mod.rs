use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

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

        println!("RPC service is running at {}", self.host);

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(e) => {
                    println!("error: get TCP stream failed, {}", e);
                    continue;
                }
            };
            self.parse_connection(stream);
        }
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
