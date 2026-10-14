use std::{
    collections::HashMap,
    io,
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
};

use tokio::net::{TcpListener, TcpStream};

use crate::p2p::{transport::Peer, transport::Transport};

pub struct TCPTransport {
    pub listener: TcpListener,
    pub peers: Mutex<HashMap<IpAddr, Box<dyn Peer>>>,
}

impl Transport for TCPTransport {}

impl TCPTransport {
    pub async fn new(listen_addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(listen_addr).await?;

        Ok(Self {
            listener,
            peers: Mutex::new(HashMap::new()),
        })
    }

    pub fn peer_count(&self) -> io::Result<usize> {
        let peers = self
            .peers
            .lock()
            .map_err(|_| io::Error::other("Peer registry mutex poisoned"))?;

        Ok(peers.len())
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub fn listen_and_accept(self: &Arc<Self>) -> io::Result<()> {
        let transport = Arc::clone(self);
        tokio::spawn(async move {
            if let Err(e) = transport.accept_loop().await {
                eprintln!("Error in accept loop: {}", e);
            }
        });

        Ok(())
    }

    pub async fn start(&self) {
        todo!()
    }

    pub async fn accept_loop(self: Arc<Self>) -> io::Result<()> {
        loop {
            match self.listener.accept().await {
                Ok((stream, addr)) => {
                    println!("Accepted connection from: {}", addr);
                    let transport = Arc::clone(&self);
                    tokio::spawn(async move {
                        transport.handle_conn(stream).await;
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                    return Err(e);
                }
            }
        }
    }

    /// Handle a new TCP connection
    pub async fn handle_conn(&self, stream: TcpStream) {
        println!("New incoming connection: {}", stream.peer_addr().unwrap());
    }
}
