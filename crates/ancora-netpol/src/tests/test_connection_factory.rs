use crate::ConnectionRequest;
#[test]
fn connection_request_stores_all_fields() {
    let req = ConnectionRequest::tcp("acme", "agent-01", "api.example.com", 8443);
    assert_eq!(req.tenant_id, "acme");
    assert_eq!(req.source, "agent-01");
    assert_eq!(req.destination_host, "api.example.com");
    assert_eq!(req.destination_port, 8443);
}
