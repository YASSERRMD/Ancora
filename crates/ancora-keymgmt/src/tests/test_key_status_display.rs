use crate::KeyStatus;
#[test]
fn status_display() {
    assert_eq!(format!("{}", KeyStatus::Active), "ACTIVE");
    assert_eq!(format!("{}", KeyStatus::Destroyed), "DESTROYED");
    assert_eq!(format!("{}", KeyStatus::Compromised), "COMPROMISED");
}
