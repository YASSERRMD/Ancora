use crate::session::ZeroTrustSession;

#[test]
fn session_boundary_expiry() {
    let s = ZeroTrustSession::new("s1", "t1", "i1", 0, 500);
    assert!(s.is_valid(499));
    assert!(!s.is_valid(500));
    assert!(!s.is_valid(501));
}

#[test]
fn revoked_session_not_valid() {
    let mut s = ZeroTrustSession::new("s1", "t1", "i1", 0, 9999);
    s.revoke();
    assert!(!s.is_valid(100));
}
