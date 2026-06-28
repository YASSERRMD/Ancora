use crate::{SealingStore, UnsealResult};
#[test]
fn unseal_fails_with_wrong_digest() {
    let mut store = SealingStore::new();
    store.seal("b1", "t1", "secret", "right", 0);
    assert_eq!(store.unseal("b1", "wrong"), UnsealResult::PolicyMismatch);
}
#[test]
fn unseal_fails_when_not_sealed() {
    let store = SealingStore::new();
    assert_eq!(store.unseal("missing", "anything"), UnsealResult::NotSealed);
}
