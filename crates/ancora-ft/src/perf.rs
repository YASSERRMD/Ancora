//! Adapter performance notes: latency estimates and throughput hints.

use crate::model::AdapterId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance characteristics noted for an adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterPerfNote {
    /// Adapter id these notes apply to.
    pub adapter_id: AdapterId,
    /// Estimated additional latency overhead in milliseconds.
    pub latency_overhead_ms: f32,
    /// Estimated memory overhead in megabytes.
    pub memory_overhead_mb: f32,
    /// Throughput degradation factor (1.0 = no degradation).
    pub throughput_factor: f32,
    /// Human-readable notes.
    pub notes: String,
}

impl AdapterPerfNote {
    pub fn new(adapter_id: AdapterId) -> Self {
        AdapterPerfNote {
            adapter_id,
            latency_overhead_ms: 0.0,
            memory_overhead_mb: 0.0,
            throughput_factor: 1.0,
            notes: String::new(),
        }
    }

    pub fn with_latency(mut self, ms: f32) -> Self {
        self.latency_overhead_ms = ms;
        self
    }

    pub fn with_memory(mut self, mb: f32) -> Self {
        self.memory_overhead_mb = mb;
        self
    }

    pub fn with_throughput_factor(mut self, factor: f32) -> Self {
        self.throughput_factor = factor;
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = notes.into();
        self
    }

    /// Whether the overhead is considered acceptable (heuristic).
    pub fn is_acceptable(&self) -> bool {
        self.latency_overhead_ms < 50.0 && self.throughput_factor > 0.8
    }
}

/// Registry of performance notes keyed by adapter id.
#[derive(Debug, Clone, Default)]
pub struct PerfNoteRegistry {
    notes: HashMap<AdapterId, AdapterPerfNote>,
}

impl PerfNoteRegistry {
    pub fn new() -> Self {
        PerfNoteRegistry {
            notes: HashMap::new(),
        }
    }

    pub fn record(&mut self, note: AdapterPerfNote) {
        self.notes.insert(note.adapter_id.clone(), note);
    }

    pub fn get(&self, id: &AdapterId) -> Option<&AdapterPerfNote> {
        self.notes.get(id)
    }

    pub fn acceptable_adapters(&self) -> Vec<&AdapterPerfNote> {
        self.notes.values().filter(|n| n.is_acceptable()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AdapterId;

    #[test]
    fn perf_note_defaults() {
        let note = AdapterPerfNote::new(AdapterId::new("a1"));
        assert_eq!(note.latency_overhead_ms, 0.0);
        assert!(note.is_acceptable());
    }

    #[test]
    fn perf_note_unacceptable_latency() {
        let note = AdapterPerfNote::new(AdapterId::new("a1"))
            .with_latency(100.0);
        assert!(!note.is_acceptable());
    }

    #[test]
    fn perf_note_registry_record_and_get() {
        let mut reg = PerfNoteRegistry::new();
        let note = AdapterPerfNote::new(AdapterId::new("a1"))
            .with_latency(10.0)
            .with_memory(128.0)
            .with_throughput_factor(0.95)
            .with_notes("good adapter");
        reg.record(note);
        let retrieved = reg.get(&AdapterId::new("a1")).unwrap();
        assert_eq!(retrieved.latency_overhead_ms, 10.0);
        assert_eq!(retrieved.notes, "good adapter");
    }

    #[test]
    fn perf_note_registry_acceptable_filter() {
        let mut reg = PerfNoteRegistry::new();
        reg.record(AdapterPerfNote::new(AdapterId::new("a1")).with_latency(10.0));
        reg.record(AdapterPerfNote::new(AdapterId::new("a2")).with_latency(200.0));
        let acceptable = reg.acceptable_adapters();
        assert_eq!(acceptable.len(), 1);
        assert_eq!(acceptable[0].adapter_id.as_str(), "a1");
    }
}
