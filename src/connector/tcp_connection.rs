use std::io::{self, prelude::*};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct client {
    ip: String,
    port: u32,
    stream: TcpStream,
}

impl client {
    pub async fn new(ip: String, port: u32) -> Self {
        let stream = TcpStream::connect(format!("{}:{}", ip, port))
            .await
            .unwrap();

        println!("connected to the server successfuly");

        return client { ip, port, stream };
    }

    pub async fn send(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data).await
    }

    pub async fn GameStart(&mut self) -> String {
        let mut buffer = [0; 128];
        self.stream.read(&mut buffer).await.unwrap();

        String::from_utf8(buffer.to_vec()).unwrap()
    }
}
