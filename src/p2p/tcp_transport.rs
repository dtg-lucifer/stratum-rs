use std::{
    collections::HashMap,
    io,
    net::{IpAddr, SocketAddr, TcpListener},
    sync::Mutex,
};

use crate::p2p::{transport::Peer, transport::Transport};

pub struct TCPTransport {
    pub listener: TcpListener,
    pub peers: Mutex<HashMap<IpAddr, Box<dyn Peer>>>,
}

impl Transport for TCPTransport {}

impl TCPTransport {
    pub fn new(listen_addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(&listen_addr).expect("Failed to bind to address");

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

    pub fn listen_and_accept(&self) {
        todo!()
    }

    pub fn start(&self) {
        todo!()
    }

    pub fn accept_loop(&self) {
        loop {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    println!("Accepted connection from: {}", addr);
                    // Here you would typically spawn a new thread or task to handle the connection
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
