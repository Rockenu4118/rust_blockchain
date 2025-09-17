use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// struct P2PInterface {
//     peers: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>,
// }

// impl P2PInterface {
//     fn new() -> Self {}

//     fn start_listener(&self, bind_addr: impl ToString) -> std::io::Result<()> {}

//     fn attach_peer(&self, mut stream: TcpStream) -> std::io::Result<()> {}

//     fn broadcast(&self, line: &str) {}
// }
