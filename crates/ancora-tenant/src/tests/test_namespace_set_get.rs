use crate::Namespace;
#[test]
fn namespace_set_and_get() {
    let mut ns = Namespace::new("tenant-a");
    ns.set("db_url", "postgres://localhost/mydb");
    assert_eq!(ns.get("db_url"), Some("postgres://localhost/mydb"));
}
#[test]
fn namespace_get_returns_none_for_missing() {
    let ns = Namespace::new("tenant-a");
    assert!(ns.get("missing_key").is_none());
}
