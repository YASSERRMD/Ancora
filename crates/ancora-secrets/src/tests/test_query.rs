use crate::{SecretKind, SecretQuery, SecretStore};
#[test]
fn query_by_kind_filters_correctly() {
    let mut store = SecretStore::new();
    store.create("t1", "db/pass", SecretKind::DatabaseCredential, "v", 1).unwrap();
    store.create("t1", "api/key", SecretKind::ApiKey, "k", 2).unwrap();
    let results = SecretQuery::new().kind(SecretKind::ApiKey).run(&store, "t1");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].path, "api/key");
}
#[test]
fn query_by_path_prefix_filters_correctly() {
    let mut store = SecretStore::new();
    store.create("t1", "db/prod/pass", SecretKind::Opaque, "v", 1).unwrap();
    store.create("t1", "api/key", SecretKind::Opaque, "k", 2).unwrap();
    store.create("t1", "db/staging/pass", SecretKind::Opaque, "s", 3).unwrap();
    let results = SecretQuery::new().path_prefix("db/").run(&store, "t1");
    assert_eq!(results.len(), 2);
}
