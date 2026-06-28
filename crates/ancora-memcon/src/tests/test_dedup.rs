use crate::dedup::Deduplicator;

#[test]
fn dedup_removes_exact_duplicates() {
    let items = vec!["a".to_string(), "b".to_string(), "a".to_string(), "c".to_string()];
    let result = Deduplicator::dedup(items);
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn dedup_preserves_order() {
    let items = vec!["c".to_string(), "a".to_string(), "b".to_string(), "a".to_string()];
    let result = Deduplicator::dedup(items);
    assert_eq!(result, vec!["c", "a", "b"]);
}

#[test]
fn dedup_by_key_works() {
    let items = vec![("k1", 1), ("k2", 2), ("k1", 3)];
    let result = Deduplicator::dedup_by_key(items, |t| t.0);
    assert_eq!(result.len(), 2);
}

#[test]
fn dedup_empty_is_safe() {
    let result = Deduplicator::dedup(vec![]);
    assert!(result.is_empty());
}
