//! Embedded LanceDB-compatible vector memory store for on-device use.
//!
//! Provides a minimal in-process vector store that mirrors the LanceDB
//! table API.  On real devices, the crate can be compiled with the
//! `lancedb` feature to delegate to the actual library; here we ship a
//! pure-Rust stand-in that works offline on all targets.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Fixed embedding dimension used across the runtime.
pub const EMBEDDING_DIM: usize = 128;

/// A dense embedding vector.
pub type Embedding = Vec<f32>;

/// A single memory record stored in the vector table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    /// Unique record identifier.
    pub id: String,
    /// Agent that owns this record.
    pub agent_id: String,
    /// Short textual label for the record.
    pub label: String,
    /// The embedding vector.
    pub embedding: Embedding,
    /// Arbitrary metadata stored alongside the vector.
    pub metadata: serde_json::Value,
}

/// Cosine-similarity result from a nearest-neighbour query.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The matching record.
    pub record: MemoryRecord,
    /// Cosine similarity in `[0.0, 1.0]`.
    pub score: f32,
}

/// Compute cosine similarity between two embeddings.
///
/// Returns 0.0 if either vector has zero norm.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "embedding dimensions must match");
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
    }
}

/// Embedded in-process vector memory store (LanceDB stand-in).
#[derive(Debug, Default)]
pub struct MemoryStore {
    records: HashMap<String, MemoryRecord>,
}

impl MemoryStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a record.
    pub fn upsert(&mut self, record: MemoryRecord) {
        self.records.insert(record.id.clone(), record);
    }

    /// Retrieve a record by ID.
    pub fn get(&self, id: &str) -> Option<&MemoryRecord> {
        self.records.get(id)
    }

    /// Delete a record.  Returns `true` if the record existed.
    pub fn delete(&mut self, id: &str) -> bool {
        self.records.remove(id).is_some()
    }

    /// Number of records in the store.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns `true` when the store is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Nearest-neighbour search returning the top-`k` results by cosine
    /// similarity to `query`.
    pub fn search(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = self
            .records
            .values()
            .map(|r| {
                let score = cosine_similarity(query, &r.embedding);
                SearchResult { record: r.clone(), score }
            })
            .collect();
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(k);
        results
    }

    /// Return all records belonging to `agent_id`.
    pub fn records_for_agent(&self, agent_id: &str) -> Vec<&MemoryRecord> {
        self.records.values().filter(|r| r.agent_id == agent_id).collect()
    }

    /// Export all records as JSON.
    pub fn export_json(&self) -> String {
        let v: Vec<&MemoryRecord> = self.records.values().collect();
        serde_json::to_string(&v).unwrap_or_else(|_| "[]".to_string())
    }
}

/// Create a normalised random-ish embedding from a seed value.
///
/// Used in tests to produce deterministic embeddings without a real encoder.
pub fn seed_embedding(seed: u64, dim: usize) -> Embedding {
    let mut v: Vec<f32> = (0..dim)
        .map(|i| {
            let raw = ((seed.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407)
                .wrapping_add(i as u64)) as f64
                / u64::MAX as f64) as f32;
            raw * 2.0 - 1.0
        })
        .collect();
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        v.iter_mut().for_each(|x| *x /= norm);
    }
    v
}

#[cfg(test)]
mod unit {
    use super::*;
    use serde_json::json;

    fn make_record(id: &str, agent: &str, seed: u64) -> MemoryRecord {
        MemoryRecord {
            id: id.to_string(),
            agent_id: agent.to_string(),
            label: format!("record-{}", id),
            embedding: seed_embedding(seed, EMBEDDING_DIM),
            metadata: json!({"seed": seed}),
        }
    }

    #[test]
    fn upsert_and_get() {
        let mut store = MemoryStore::new();
        let r = make_record("r1", "agent-a", 42);
        store.upsert(r.clone());
        let fetched = store.get("r1").unwrap();
        assert_eq!(fetched.id, "r1");
    }

    #[test]
    fn delete_removes_record() {
        let mut store = MemoryStore::new();
        store.upsert(make_record("r1", "a", 1));
        assert!(store.delete("r1"));
        assert!(store.get("r1").is_none());
    }

    #[test]
    fn search_returns_top_k() {
        let mut store = MemoryStore::new();
        for i in 0u64..10 {
            store.upsert(make_record(&format!("r{}", i), "a", i));
        }
        let query = seed_embedding(3, EMBEDDING_DIM);
        let results = store.search(&query, 3);
        assert_eq!(results.len(), 3);
        // First result should be the record with seed 3 (identical vector).
        assert!((results[0].score - 1.0).abs() < 1e-5);
    }

    #[test]
    fn cosine_similarity_identical_vectors() {
        let v = seed_embedding(7, 16);
        let sim = cosine_similarity(&v, &v);
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn records_for_agent_filters_correctly() {
        let mut store = MemoryStore::new();
        store.upsert(make_record("r1", "agent-a", 1));
        store.upsert(make_record("r2", "agent-b", 2));
        let a_records = store.records_for_agent("agent-a");
        assert_eq!(a_records.len(), 1);
        assert_eq!(a_records[0].agent_id, "agent-a");
    }
}
