use crate::Namespace;
#[test]
fn empty_namespace_count_is_zero() {
    let ns = Namespace::new("t1");
    assert_eq!(ns.count(), 0);
}
#[test]
fn count_reflects_inserted_keys() {
    let mut ns = Namespace::new("t1");
    ns.set("a", "1");
    ns.set("b", "2");
    ns.set("c", "3");
    assert_eq!(ns.count(), 3);
}
