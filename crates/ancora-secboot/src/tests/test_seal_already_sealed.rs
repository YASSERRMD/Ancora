use crate::{SealResult, SealingStore};
#[test]
fn sealing_same_id_twice_returns_already_sealed() {
    let mut store = SealingStore::new();
    store.seal("b1", "t1", "s1", "d1", 0);
    let result = store.seal("b1", "t1", "s2", "d2", 1);
    assert_eq!(result, SealResult::AlreadySealed);
    assert_eq!(store.count(), 1);
}
