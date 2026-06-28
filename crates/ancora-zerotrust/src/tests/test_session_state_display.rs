use crate::session::SessionState;

#[test]
fn state_display() {
    assert_eq!(format!("{}", SessionState::Active), "ACTIVE");
    assert_eq!(format!("{}", SessionState::Expired), "EXPIRED");
    assert_eq!(format!("{}", SessionState::Revoked), "REVOKED");
}
