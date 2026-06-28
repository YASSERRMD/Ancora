use crate::session::SessionState;

#[test]
fn display_open() {
    assert_eq!(format!("{}", SessionState::Open), "OPEN");
}

#[test]
fn display_logged_in() {
    assert_eq!(format!("{}", SessionState::LoggedIn), "LOGGED_IN");
}

#[test]
fn display_logged_out() {
    assert_eq!(format!("{}", SessionState::LoggedOut), "LOGGED_OUT");
}

#[test]
fn display_closed() {
    assert_eq!(format!("{}", SessionState::Closed), "CLOSED");
}
