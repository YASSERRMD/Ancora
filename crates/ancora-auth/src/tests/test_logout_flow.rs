use crate::{
    logout_all_for_subject, logout_session, RevocationStore, Session, SessionState, SessionStore,
};

fn setup() -> (SessionStore, RevocationStore) {
    let mut sessions = SessionStore::new();
    sessions.create(Session::new("s1", "t", "alice", "tok-alice-1", 0, 500));
    sessions.create(Session::new("s2", "t", "alice", "tok-alice-2", 0, 500));
    sessions.create(Session::new("s3", "t", "bob", "tok-bob-1", 0, 500));
    let revocations = RevocationStore::new();
    (sessions, revocations)
}

#[test]
fn logout_session_marks_logged_out_and_revokes_token() {
    let (mut sessions, mut revocations) = setup();
    let result = logout_session(&mut sessions, &mut revocations, "s1");
    assert!(result.session_logged_out);
    assert!(result.token_revoked);
    assert!(revocations.is_revoked("tok-alice-1"));
    let s = sessions.get("s1").unwrap();
    assert_eq!(s.state, SessionState::LoggedOut);
}

#[test]
fn logout_all_for_subject_revokes_all_tokens() {
    let (mut sessions, mut revocations) = setup();
    let count = logout_all_for_subject(&mut sessions, &mut revocations, "alice");
    assert_eq!(count, 2);
    assert!(revocations.is_revoked("tok-alice-1"));
    assert!(revocations.is_revoked("tok-alice-2"));
    assert!(!revocations.is_revoked("tok-bob-1"));
}

#[test]
fn logout_nonexistent_session() {
    let (mut sessions, mut revocations) = setup();
    let result = logout_session(&mut sessions, &mut revocations, "does-not-exist");
    assert!(!result.session_logged_out);
    assert!(!result.token_revoked);
}
