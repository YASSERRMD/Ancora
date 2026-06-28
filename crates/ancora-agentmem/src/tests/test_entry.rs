use crate::entry::{MemoryEntry, MemoryKind};

#[test]
fn importance_clamped_to_10() {
    let e = MemoryEntry::new("1", MemoryKind::Fact, "rust is fast", 99, 0);
    assert_eq!(e.importance, 10);
}

#[test]
fn access_increments_count() {
    let mut e = MemoryEntry::new("1", MemoryKind::Fact, "content", 5, 0);
    e.access(10);
    assert_eq!(e.access_count, 1);
    assert_eq!(e.last_accessed, 10);
}

#[test]
fn score_higher_for_recent_entries() {
    let mut recent = MemoryEntry::new("r", MemoryKind::Fact, "recent", 5, 1000);
    let old = MemoryEntry::new("o", MemoryKind::Fact, "old", 5, 0);
    recent.access(1000);
    assert!(recent.score(1000) > old.score(1000));
}
