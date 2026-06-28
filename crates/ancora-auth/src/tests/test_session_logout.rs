use crate::{Session, SessionState, SessionStore};

#[test]
fn logout_nonexistent_session_returns_false() {
    let mut store = SessionStore::new();
    assert!(!store.logout("not-here"));
}

#[test]
fn sessions_for_subject_returns_all() {
    let mut store = SessionStore::new();
    store.create(Session::new("s1", "t", "alice", "tok1", 0, 500));
    store.create(Session::new("s2", "t", "alice", "tok2", 0, 500));
    store.create(Session::new("s3", "t", "bob", "tok3", 0, 500));
    let alice_sessions = store.sessions_for_subject("alice");
    assert_eq!(alice_sessions.len(), 2);
}

#[test]
fn logged_out_session_not_in_active_count() {
    let mut store = SessionStore::new();
    store.create(Session::new("s4", "t", "carol", "tok4", 0, 500));
    store.create(Session::new("s5", "t", "carol", "tok5", 0, 500));
    assert_eq!(store.active_count(100), 2);
    store.logout("s4");
    assert_eq!(store.active_count(100), 1);
}

#[test]
fn session_metadata_stored() {
    let session = Session::new("s6", "t", "dana", "tok6", 0, 500)
        .with_metadata("ip", "10.0.0.1")
        .with_metadata("user_agent", "agent/1.0");
    assert_eq!(session.metadata.get("ip"), Some(&"10.0.0.1".to_string()));
    assert_eq!(session.state, SessionState::Active);
}
