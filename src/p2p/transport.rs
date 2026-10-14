/// Peer is an interface that represents the remote node
/// in a network
pub trait Peer: Send + Sync {}

/// Transport is anything that handles the connection
/// between the nodes in the network
///
/// This can be of form (TCP, UDP and WebSockets)
pub trait Transport {}
