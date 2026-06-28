use std::collections::HashMap;
use crate::entry::{MemoryEntry, MemoryKind};

pub struct MemoryStore {
    entries: HashMap<String, MemoryEntry>,
    pub max_entries: usize,
}

impl MemoryStore {
    pub fn new(max_entries: usize) -> Self {
        Self { entries: HashMap::new(), max_entries }
    }

    pub fn insert(&mut self, entry: MemoryEntry, now: u64) {
        if self.entries.len() >= self.max_entries {
            self.evict_lowest_score(now);
        }
        self.entries.insert(entry.id.clone(), entry);
    }

    pub fn get(&mut self, id: &str, now: u64) -> Option<&MemoryEntry> {
        if let Some(e) = self.entries.get_mut(id) {
            e.access(now);
        }
        self.entries.get(id)
    }

    pub fn by_kind(&self, kind: &MemoryKind) -> Vec<&MemoryEntry> {
        self.entries.values().filter(|e| &e.kind == kind).collect()
    }

    pub fn top_k(&self, k: usize, now: u64) -> Vec<&MemoryEntry> {
        let mut scored: Vec<(&str, f64)> = self.entries
            .iter()
            .map(|(id, e)| (id.as_str(), e.score(now)))
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored.iter().take(k).filter_map(|(id, _)| self.entries.get(*id)).collect()
    }

    pub fn remove(&mut self, id: &str) {
        self.entries.remove(id);
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    fn evict_lowest_score(&mut self, now: u64) {
        if let Some(id) = self.entries
            .iter()
            .min_by(|a, b| a.1.score(now).partial_cmp(&b.1.score(now)).unwrap())
            .map(|(id, _)| id.clone())
        {
            self.entries.remove(&id);
        }
    }
}
