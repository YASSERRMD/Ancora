use crate::entry::{MemoryEntry, MemoryKind};
use crate::store::MemoryStore;

fn entry(id: &str, importance: u8) -> MemoryEntry {
    MemoryEntry::new(id, MemoryKind::Fact, id, importance, 0)
}

#[test]
fn insert_and_count() {
    let mut s = MemoryStore::new(10);
    s.insert(entry("a", 5), 0);
    s.insert(entry("b", 3), 0);
    assert_eq!(s.count(), 2);
}

#[test]
fn evicts_lowest_score_when_full() {
    let mut s = MemoryStore::new(2);
    s.insert(entry("high", 9), 0);
    s.insert(entry("low", 1), 0);
    s.insert(entry("new", 5), 0); // triggers eviction of "low"
    assert_eq!(s.count(), 2);
    assert!(s.by_kind(&MemoryKind::Fact).iter().any(|e| e.id == "high"));
}

#[test]
fn top_k_returns_highest_scored() {
    let mut s = MemoryStore::new(10);
    for i in 1u8..=5 {
        s.insert(entry(&i.to_string(), i), 0);
    }
    let top = s.top_k(2, 0);
    assert_eq!(top.len(), 2);
}
