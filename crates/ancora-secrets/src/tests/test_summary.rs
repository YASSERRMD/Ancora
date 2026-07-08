use crate::{SecretKind, SecretStore, SecretSummary};
#[test]
fn summary_counts_total_secrets() {
    let mut store = SecretStore::new();
    store
        .create("t1", "db/pass", SecretKind::Opaque, "v", 1)
        .unwrap();
    store
        .create("t1", "api/key", SecretKind::ApiKey, "k", 2)
        .unwrap();
    let summary = SecretSummary::for_tenant(&store, "t1");
    assert_eq!(summary.total, 2);
}
#[test]
fn summary_counts_with_ttl() {
    let mut store = SecretStore::new();
    store
        .create("t1", "db/pass", SecretKind::Opaque, "v", 1)
        .unwrap();
    store
        .create("t1", "tmp/key", SecretKind::Opaque, "k", 2)
        .unwrap();
    store.read_mut("t1", "tmp/key").unwrap().ttl_ticks = Some(100);
    let summary = SecretSummary::for_tenant(&store, "t1");
    assert_eq!(summary.with_ttl, 1);
}
#[test]
fn summary_total_versions() {
    let mut store = SecretStore::new();
    store
        .create("t1", "db/pass", SecretKind::Opaque, "v1", 1)
        .unwrap();
    store.write_version("t1", "db/pass", "v2", 2).unwrap();
    store
        .create("t1", "api/key", SecretKind::ApiKey, "k", 3)
        .unwrap();
    let summary = SecretSummary::for_tenant(&store, "t1");
    assert_eq!(summary.total_versions(), 3);
}
