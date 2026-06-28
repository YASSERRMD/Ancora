use crate::validate_path;
#[test]
fn space_in_path_rejected() {
    assert!(validate_path("db password").is_err());
}
#[test]
fn special_chars_rejected() {
    assert!(validate_path("db$key").is_err());
    assert!(validate_path("key@domain").is_err());
    assert!(validate_path("key!").is_err());
}
