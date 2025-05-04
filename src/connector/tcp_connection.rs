use std::io::{self, prelude::*};
use std::net::TcpStream;

pub struct client {
    ip: String,
    port: u32,
    stream: TcpStream,
}

impl client {
    pub fn new(ip: String, port: u32) -> Self {
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();

        println!("connected to the server successfuly");

        return client {
            ip,
            port,
            stream: stream,
        };
    }

    pub fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        return self.stream.write(data);
    }
}
