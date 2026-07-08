use crate::{EvidenceItem, EvidenceKind, EvidenceStore};
#[test]
fn for_tenant_filters_by_tenant() {
    let mut store = EvidenceStore::new();
    store.insert(EvidenceItem::new(
        "e1",
        EvidenceKind::LogEntry,
        "t1 log",
        "d",
        1,
        "t1",
    ));
    store.insert(EvidenceItem::new(
        "e2",
        EvidenceKind::LogEntry,
        "t2 log",
        "d",
        2,
        "t2",
    ));
    assert_eq!(store.for_tenant("t1").len(), 1);
    assert_eq!(store.for_tenant("t2").len(), 1);
}
