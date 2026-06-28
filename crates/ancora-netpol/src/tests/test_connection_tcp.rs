use crate::{ConnectionRequest, Protocol};
#[test]
fn tcp_factory_sets_protocol() {
    let req = ConnectionRequest::tcp("t1", "agent-01", "api.example.com", 443);
    assert_eq!(req.protocol, Protocol::Tcp);
    assert_eq!(req.destination_port, 443);
    assert_eq!(req.destination_host, "api.example.com");
}
