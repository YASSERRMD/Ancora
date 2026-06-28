use crate::{EvidenceItem, EvidenceKind};
#[test]
fn evidence_item_stores_all_fields() {
    let e = EvidenceItem::new("ev-001", EvidenceKind::LogEntry, "Auth log", "Server auth log", 10, "t1");
    assert_eq!(e.id, "ev-001");
    assert_eq!(e.kind, EvidenceKind::LogEntry);
    assert_eq!(e.collected_tick, 10);
    assert_eq!(e.tenant_id, "t1");
}
