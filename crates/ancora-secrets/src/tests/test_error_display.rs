use crate::SecretError;
#[test]
fn not_found_display() {
    let e = SecretError::NotFound("db/pass".into());
    assert!(format!("{}", e).contains("db/pass"));
}
#[test]
fn invalid_path_display() {
    let e = SecretError::InvalidPath("must not contain spaces".into());
    assert!(format!("{}", e).contains("spaces"));
}
