#[cfg(test)]
mod tests {
    use std::{
        net::{IpAddr, Ipv4Addr, SocketAddr},
        sync::Arc,
        time::Duration,
    };

    use crate::p2p::{tcp_transport::TCPTransport, transport::Peer};

    struct MockPeer;
    impl Peer for MockPeer {}

    #[tokio::test]
    async fn test_new_transport() {
        let transport = TCPTransport::new("127.0.0.1:0").await.unwrap();
        assert_eq!(transport.peer_count().unwrap(), 0);
    }

    #[tokio::test]
    async fn test_new_transport_invalid_address() {
        let result = TCPTransport::new("999.999.999.999:9999").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_local_addr() {
        let transport = TCPTransport::new("127.0.0.1:0").await.unwrap();
        let addr = transport.local_addr().unwrap();
        assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert!(addr.port() > 0);
    }

    #[tokio::test]
    async fn test_peer_count_and_registration() {
        let transport = TCPTransport::new("127.0.0.1:0").await.unwrap();
        assert_eq!(transport.peer_count().unwrap(), 0);

        {
            let mut peers = transport.peers.lock().unwrap();
            let peer_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
            peers.insert(peer_ip, Box::new(MockPeer));
        }

        assert_eq!(transport.peer_count().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_listen_and_accept() {
        let transport = TCPTransport::new("127.0.0.1:0").await.unwrap();
        let transport_arc = Arc::new(transport);

        let result = transport_arc.listen_and_accept();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_accept_incoming_connection() {
        let transport = TCPTransport::new("127.0.0.1:0").await.unwrap();
        let local_addr = transport.local_addr().unwrap();
        let transport_arc = Arc::new(transport);

        transport_arc.listen_and_accept().unwrap();

        // Connect a client to verify accept_loop accepts connections
        let client_stream = tokio::net::TcpStream::connect(local_addr).await;
        assert!(client_stream.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_concurrent_connections() {
        let transport = TCPTransport::new("127.0.0.1:0").await.unwrap();
        let local_addr = transport.local_addr().unwrap();
        let transport_arc = Arc::new(transport);

        transport_arc.listen_and_accept().unwrap();

        // Spawn multiple clients connecting to the transport concurrently
        let mut handles = Vec::new();
        for _ in 0..5 {
            let addr: SocketAddr = local_addr;
            let handle = tokio::spawn(async move { tokio::net::TcpStream::connect(addr).await });
            handles.push(handle);
        }

        for handle in handles {
            let res = handle.await.unwrap();
            assert!(res.is_ok());
        }

        // Allow background tasks a brief moment to process
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
