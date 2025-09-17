use bincode;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::message::Message;

pub struct Client {
    addr: String,
    stream: TcpStream,
}

impl Client {
    pub fn connect(addr: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        println!("Connected to {}", addr);
        Ok(Self {
            addr: addr.to_string(),
            stream,
        })
    }

    pub fn send(&mut self, msg: &str) -> std::io::Result<()> {
        self.stream.write_all(msg.as_bytes())
    }

    // pub fn send_message(mut stream: TcpStream, msg: Message) -> bincode::Result<Message> {
    //     // let options = bincode::Encode(&msg);
    //     // let bytes = bincode::serial
    //     let bytes = bincode::serialize(&msg)?;
    //     // Ok(())
    // }

    // pub fn listen(&mut self) -> std::io::Result<()> {}
}
