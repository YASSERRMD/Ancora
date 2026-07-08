use crate::request::AccessRequest;

#[test]
fn new_request() {
    let r = AccessRequest::new("r1", "t1", "i1", "db/users", "READ", 100);
    assert_eq!(r.id, "r1");
    assert!(r.device_id.is_none());
    assert!(r.context.is_empty());
}

#[test]
fn request_with_device() {
    let r = AccessRequest::new("r1", "t1", "i1", "db", "WRITE", 1).with_device("d1");
    assert_eq!(r.device_id.as_deref(), Some("d1"));
}

#[test]
fn request_with_context() {
    let r = AccessRequest::new("r1", "t1", "i1", "res", "ACT", 1).with_context("ip", "10.0.0.1");
    assert_eq!(r.context.get("ip").map(|s| s.as_str()), Some("10.0.0.1"));
}
