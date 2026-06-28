use crate::validate_path;
#[test]
fn leading_slash_is_rejected() {
    assert!(validate_path("/db/pass").is_err());
}
#[test]
fn trailing_slash_is_rejected() {
    assert!(validate_path("db/pass/").is_err());
}
