use crate::validate_path;
#[test]
fn empty_path_is_rejected() {
    assert!(validate_path("").is_err());
}
