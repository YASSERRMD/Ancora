use crate::SecretStatus;
#[test]
fn status_variants_are_distinct() {
    assert_ne!(SecretStatus::Active, SecretStatus::Rotated);
    assert_ne!(SecretStatus::Rotated, SecretStatus::Deleted);
    assert_ne!(SecretStatus::Active, SecretStatus::Expired);
}
