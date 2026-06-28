use crate::session::{SessionState, SessionStore, ZeroTrustSession};

#[test]
fn session_valid_before_expiry() {
    let s = ZeroTrustSession::new("s1", "t1", "i1", 0, 1000);
    assert!(s.is_valid(500));
    assert!(!s.is_valid(1000));
}

#[test]
fn session_expire() {
    let mut s = ZeroTrustSession::new("s1", "t1", "i1", 0, 9999);
    s.expire();
    assert!(!s.is_valid(100));
    assert_eq!(s.state, SessionState::Expired);
}

#[test]
fn session_revoke() {
    let mut s = ZeroTrustSession::new("s1", "t1", "i1", 0, 9999);
    s.revoke();
    assert_eq!(s.state, SessionState::Revoked);
}

#[test]
fn session_store_active() {
    let mut store = SessionStore::new();
    store.insert(ZeroTrustSession::new("s1", "t1", "i1", 0, 1000));
    store.insert(ZeroTrustSession::new("s2", "t1", "i2", 0, 100));
    assert_eq!(store.active(500).len(), 1);
    assert_eq!(store.active(50).len(), 2);
}

#[test]
fn session_refresh_verification() {
    let mut s = ZeroTrustSession::new("s1", "t1", "i1", 0, 9999);
    s.refresh_verification(500);
    assert_eq!(s.last_verified_tick, 500);
}
