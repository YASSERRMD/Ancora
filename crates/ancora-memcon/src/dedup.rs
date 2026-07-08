use std::collections::HashSet;

/// Removes duplicate memories by exact content match.
pub struct Deduplicator;

impl Deduplicator {
    pub fn dedup(items: Vec<String>) -> Vec<String> {
        let mut seen = HashSet::new();
        items
            .into_iter()
            .filter(|s| seen.insert(s.clone()))
            .collect()
    }

    pub fn dedup_by_key<T, K, F>(items: Vec<T>, key_fn: F) -> Vec<T>
    where
        K: Eq + std::hash::Hash,
        F: Fn(&T) -> K,
    {
        let mut seen = HashSet::new();
        items
            .into_iter()
            .filter(|item| seen.insert(key_fn(item)))
            .collect()
    }
}
