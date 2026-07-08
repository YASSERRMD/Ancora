use crate::entry::{MemoryEntry, MemoryKind};
use crate::retrieval::KeywordRetriever;

fn mem(id: &str, content: &str) -> MemoryEntry {
    MemoryEntry::new(id, MemoryKind::Fact, content, 5, 0)
}

#[test]
fn retrieves_matching_memory() {
    let entries = [mem("1", "rust is fast"), mem("2", "python is slow")];
    let refs: Vec<&MemoryEntry> = entries.iter().collect();
    let results = KeywordRetriever::retrieve(&refs, "rust", 5);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "1");
}

#[test]
fn multi_word_query_ranks_better_match_first() {
    let entries = [
        mem("1", "rust is fast"),
        mem("2", "rust and python comparison"),
    ];
    let refs: Vec<&MemoryEntry> = entries.iter().collect();
    let results = KeywordRetriever::retrieve(&refs, "rust python", 5);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].id, "2"); // matches both words
}

#[test]
fn no_match_returns_empty() {
    let entries = [mem("1", "rust is fast")];
    let refs: Vec<&MemoryEntry> = entries.iter().collect();
    let results = KeywordRetriever::retrieve(&refs, "java", 5);
    assert!(results.is_empty());
}
