use crate::validate_path;
#[test]
fn valid_paths_accepted() {
    assert!(validate_path("db/password").is_ok());
    assert!(validate_path("service/api-key").is_ok());
    assert!(validate_path("secrets/prod/tls.cert").is_ok());
    assert!(validate_path("a").is_ok());
    assert!(validate_path("a_b_c").is_ok());
}
