#[cfg(test)]
mod tests {
    use crate::p2p::tcp_transport::TCPTransport;

    #[test]
    fn test_new_transport() {
        let transport = TCPTransport::new("127.0.0.1:0").unwrap();
        assert_eq!(transport.peer_count().unwrap(), 0);
    }

    #[test]
    fn test_local_addr() {
        let transport = TCPTransport::new("127.0.0.1:0").unwrap();
        assert!(transport.local_addr().is_ok());
    }
}
