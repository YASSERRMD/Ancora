use crate::{Session, SessionState, SessionStore};

#[test]
fn session_is_active_before_expiry() {
    let session = Session::new("s1", "tenant-a", "alice", "tok-1", 0, 500);
    assert!(session.is_valid(100));
    assert_eq!(session.state, SessionState::Active);
}

#[test]
fn session_expired_is_invalid() {
    let session = Session::new("s2", "tenant-a", "alice", "tok-2", 0, 100);
    assert!(!session.is_valid(200));
}

#[test]
fn session_store_logout_marks_logged_out() {
    let mut store = SessionStore::new();
    let session = Session::new("s3", "tenant-a", "alice", "tok-3", 0, 500);
    store.create(session);
    assert!(store.logout("s3"));
    let s = store.get("s3").expect("exists");
    assert_eq!(s.state, SessionState::LoggedOut);
    assert!(!s.is_valid(100));
}

#[test]
fn session_store_active_count() {
    let mut store = SessionStore::new();
    store.create(Session::new("s4", "t", "u", "t4", 0, 500));
    store.create(Session::new("s5", "t", "u", "t5", 0, 50));
    assert_eq!(store.active_count(100), 1);
}
