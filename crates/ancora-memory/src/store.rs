use std::collections::HashMap;
use std::sync::Mutex;

use crate::entry::MemoryEntry;
use crate::scope::Scope;
use crate::tier::MemoryTier;
use crate::traits::Memory;

/// In-memory backend that stores entries keyed by scope.
pub struct InMemoryStore {
    data: Mutex<HashMap<Scope, Vec<MemoryEntry>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self { data: Mutex::new(HashMap::new()) }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for InMemoryStore {
    fn write(&self, scope: &Scope, entry: MemoryEntry) {
        self.data.lock().unwrap().entry(scope.clone()).or_default().push(entry);
    }

    fn read(&self, scope: &Scope, tier: Option<MemoryTier>) -> Vec<MemoryEntry> {
        let guard = self.data.lock().unwrap();
        let entries = guard.get(scope).cloned().unwrap_or_default();
        match tier {
            Some(t) => entries.into_iter().filter(|e| e.tier == t).collect(),
            None => entries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoped_writes_isolate_by_resource() {
        let store = InMemoryStore::new();
        let scope_a = Scope::new("res-a", "t1");
        let scope_b = Scope::new("res-b", "t1");
        store.write(&scope_a, MemoryEntry::new("k1", "v1", MemoryTier::Working));
        store.write(&scope_b, MemoryEntry::new("k2", "v2", MemoryTier::Working));
        assert_eq!(store.read(&scope_a, None).len(), 1);
        assert_eq!(store.read(&scope_b, None).len(), 1);
        assert_eq!(store.read(&scope_a, None)[0].key, "k1");
    }

    #[test]
    fn scoped_writes_isolate_by_thread() {
        let store = InMemoryStore::new();
        let scope_t1 = Scope::new("res-x", "thread-1");
        let scope_t2 = Scope::new("res-x", "thread-2");
        store.write(&scope_t1, MemoryEntry::new("a", "1", MemoryTier::Episodic));
        store.write(&scope_t1, MemoryEntry::new("b", "2", MemoryTier::Episodic));
        store.write(&scope_t2, MemoryEntry::new("c", "3", MemoryTier::Episodic));
        assert_eq!(store.read(&scope_t1, None).len(), 2);
        assert_eq!(store.read(&scope_t2, None).len(), 1);
    }

    #[test]
    fn tier_filter_returns_only_matching_entries() {
        let store = InMemoryStore::new();
        let scope = Scope::new("res-y", "t1");
        store.write(&scope, MemoryEntry::new("w", "1", MemoryTier::Working));
        store.write(&scope, MemoryEntry::new("e", "2", MemoryTier::Episodic));
        store.write(&scope, MemoryEntry::new("s", "3", MemoryTier::Semantic));
        store.write(&scope, MemoryEntry::new("a", "4", MemoryTier::Archival));
        assert_eq!(store.read(&scope, Some(MemoryTier::Working)).len(), 1);
        assert_eq!(store.read(&scope, Some(MemoryTier::Episodic)).len(), 1);
        assert_eq!(store.read(&scope, None).len(), 4);
    }
}
