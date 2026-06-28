use crate::identity::IdentityStatus;

#[test]
fn status_display() {
    assert_eq!(format!("{}", IdentityStatus::Active), "ACTIVE");
    assert_eq!(format!("{}", IdentityStatus::Suspended), "SUSPENDED");
    assert_eq!(format!("{}", IdentityStatus::Revoked), "REVOKED");
}
