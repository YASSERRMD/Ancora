use crate::session::{HsmSession, SessionState};

#[test]
fn session_open() {
    let s = HsmSession::new(1, 0, true, 100);
    assert_eq!(s.state, SessionState::Open);
    assert!(s.is_active());
    assert!(!s.is_logged_in());
}

#[test]
fn session_login_logout() {
    let mut s = HsmSession::new(1, 0, true, 1);
    s.login();
    assert!(s.is_logged_in());
    s.logout();
    assert!(!s.is_logged_in());
    assert!(s.is_active());
}

#[test]
fn session_close() {
    let mut s = HsmSession::new(1, 0, false, 1);
    s.close();
    assert!(!s.is_active());
    assert_eq!(s.state, SessionState::Closed);
}
