use crate::{EvidenceItem, EvidenceKind, EvidenceStore};
#[test]
fn store_insert_and_count() {
    let mut store = EvidenceStore::new();
    store.insert(EvidenceItem::new("ev-001", EvidenceKind::TestResult, "Test", "desc", 1, "t1"));
    assert_eq!(store.count(), 1);
    assert!(store.get("ev-001").is_some());
    assert!(store.get("missing").is_none());
}
