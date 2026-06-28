use crate::{SecretKind, SecretStore};
#[test]
fn list_tenant_returns_only_that_tenants_secrets() {
    let mut store = SecretStore::new();
    store.create("t1", "db/pass", SecretKind::Opaque, "v", 1).unwrap();
    store.create("t2", "api/key", SecretKind::ApiKey, "k", 2).unwrap();
    store.create("t1", "ssh/key", SecretKind::SshKey, "k", 3).unwrap();
    let t1 = store.list_tenant("t1");
    assert_eq!(t1.len(), 2);
    assert!(t1.iter().all(|s| s.tenant_id == "t1"));
}
