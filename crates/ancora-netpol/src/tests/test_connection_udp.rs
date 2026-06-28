use crate::{ConnectionRequest, Protocol};
#[test]
fn udp_factory_sets_protocol() {
    let req = ConnectionRequest::udp("t1", "agent-01", "dns.internal", 53);
    assert_eq!(req.protocol, Protocol::Udp);
    assert_eq!(req.destination_port, 53);
}
