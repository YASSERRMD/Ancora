use crate::Namespace;
#[test]
fn namespace_remove_deletes_key() {
    let mut ns = Namespace::new("t1");
    ns.set("key", "val");
    assert!(ns.contains("key"));
    let removed = ns.remove("key");
    assert_eq!(removed, Some("val".to_string()));
    assert!(!ns.contains("key"));
}
#[test]
fn namespace_remove_missing_returns_none() {
    let mut ns = Namespace::new("t1");
    assert_eq!(ns.remove("ghost"), None);
}
