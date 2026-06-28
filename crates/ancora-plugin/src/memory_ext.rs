/// Memory extension point - persist and retrieve agent memory.

use std::collections::HashMap;

/// A single memory entry.
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
    /// Arbitrary tags for filtering.
    pub tags: Vec<String>,
    /// Unix timestamp of last write (seconds).
    pub updated_at: u64,
}

/// Parameters for a memory lookup.
#[derive(Debug, Clone)]
pub struct MemoryQuery {
    /// If set, only return entries with all of these tags.
    pub tags: Vec<String>,
    /// If set, only return up to this many entries.
    pub limit: Option<usize>,
    /// Optional prefix filter on keys.
    pub key_prefix: Option<String>,
}

/// Errors from the memory back-end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    NotFound(String),
    SerializationFailed(String),
    StorageFull,
    Unknown(String),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::NotFound(k) => write!(f, "key not found: {k}"),
            MemoryError::SerializationFailed(s) => write!(f, "serialization failed: {s}"),
            MemoryError::StorageFull => write!(f, "memory storage is full"),
            MemoryError::Unknown(s) => write!(f, "unknown error: {s}"),
        }
    }
}

impl std::error::Error for MemoryError {}

/// Trait that memory plugins must implement.
pub trait MemoryPlugin: Send + Sync {
    fn memory_id(&self) -> &str;

    /// Persist a key-value pair with optional tags.
    fn write(&mut self, entry: MemoryEntry) -> Result<(), MemoryError>;

    /// Retrieve a specific entry by key.
    fn read(&self, key: &str) -> Result<MemoryEntry, MemoryError>;

    /// Delete an entry by key.
    fn delete(&mut self, key: &str) -> Result<(), MemoryError>;

    /// Search entries matching the query.
    fn search(&self, query: MemoryQuery) -> Result<Vec<MemoryEntry>, MemoryError>;

    /// Return the number of entries currently stored.
    fn count(&self) -> usize;
}

/// In-memory implementation for testing.
pub struct HashMapMemory {
    id: String,
    store: HashMap<String, MemoryEntry>,
}

impl HashMapMemory {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), store: HashMap::new() }
    }
}

impl MemoryPlugin for HashMapMemory {
    fn memory_id(&self) -> &str {
        &self.id
    }

    fn write(&mut self, entry: MemoryEntry) -> Result<(), MemoryError> {
        self.store.insert(entry.key.clone(), entry);
        Ok(())
    }

    fn read(&self, key: &str) -> Result<MemoryEntry, MemoryError> {
        self.store
            .get(key)
            .cloned()
            .ok_or_else(|| MemoryError::NotFound(key.to_string()))
    }

    fn delete(&mut self, key: &str) -> Result<(), MemoryError> {
        if self.store.remove(key).is_none() {
            return Err(MemoryError::NotFound(key.to_string()));
        }
        Ok(())
    }

    fn search(&self, query: MemoryQuery) -> Result<Vec<MemoryEntry>, MemoryError> {
        let mut results: Vec<MemoryEntry> = self
            .store
            .values()
            .filter(|e| {
                let tag_ok = query.tags.iter().all(|t| e.tags.contains(t));
                let prefix_ok = query
                    .key_prefix
                    .as_ref()
                    .map(|p| e.key.starts_with(p.as_str()))
                    .unwrap_or(true);
                tag_ok && prefix_ok
            })
            .cloned()
            .collect();

        results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    fn count(&self) -> usize {
        self.store.len()
    }
}
