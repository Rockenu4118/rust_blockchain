use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    pub addr: String,
    pub peers: Arc<Mutex<Vec<TcpStream>>>,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        println!("Server listening on {}", self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection from: {}", stream.peer_addr()?);

                    let peers = Arc::clone(&self.peers);

                    {
                        let mut peers_guard = peers.lock().unwrap();
                        peers_guard.push(stream.try_clone()?);
                    }

                    // handle messages
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_client(stream, peers) {
                            eprintln!("Client handler error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
                _ => eprintln!("error"),
            }
        }

        Ok(())
    }

    fn handle_client(
        mut stream: TcpStream,
        peers: Arc<Mutex<Vec<TcpStream>>>,
    ) -> std::io::Result<()> {
        let mut buf = [0; 1024];

        loop {
            let n = stream.read(&mut buf)?;
            if n == 0 {
                println!("Client {} disconnected", stream.peer_addr()?);

                return Ok(());
            }

            let msg = String::from_utf8_lossy(&buf[..n]).to_string();
            print!("Recieved from {}: {}", stream.peer_addr()?, msg);
        }
    }
}
