use crate::Namespace;
#[test]
fn scoped_key_includes_tenant_prefix() {
    let ns = Namespace::new("acme");
    let key = ns.scoped_key("db_password");
    assert_eq!(key, "acme::db_password");
}
