use crate::{SealResult, SealingStore, UnsealResult};
#[test]
fn seal_and_unseal_with_correct_digest() {
    let mut store = SealingStore::new();
    let result = store.seal("b1", "t1", "my-secret", "correct-digest", 0);
    assert_eq!(result, SealResult::Sealed);
    let unseal = store.unseal("b1", "correct-digest");
    assert_eq!(unseal, UnsealResult::Unsealed("my-secret".to_string()));
}
