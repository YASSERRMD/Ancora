use crate::Namespace;
#[test]
fn different_tenant_namespaces_are_isolated() {
    let ns_a = Namespace::new("tenant-a");
    let ns_b = Namespace::new("tenant-b");
    assert!(ns_a.is_isolated_from(&ns_b));
}
#[test]
fn same_tenant_namespace_is_not_isolated_from_itself() {
    let ns_a1 = Namespace::new("tenant-a");
    let ns_a2 = Namespace::new("tenant-a");
    assert!(!ns_a1.is_isolated_from(&ns_a2));
}
