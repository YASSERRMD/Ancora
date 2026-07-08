use crate::aggregate::AggregateMetrics;
use crate::executor::RunId;
/// Eval run persistence - serialize and deserialize eval run records.
///
/// Uses a simple newline-delimited JSON-like text format (no external deps).
use std::fmt;

/// A persisted record of a completed eval run.
#[derive(Debug, Clone)]
pub struct EvalRunRecord {
    pub run_id: RunId,
    pub timestamp: u64,
    pub suite_name: String,
    pub n_rollouts: usize,
    pub metrics: AggregateMetrics,
}

impl fmt::Display for EvalRunRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "run_id={} ts={} suite={} n={} pass_rate={:.4} ci=[{:.4},{:.4}]",
            self.run_id.0,
            self.timestamp,
            self.suite_name,
            self.n_rollouts,
            self.metrics.pass_rate,
            self.metrics.ci_lower,
            self.metrics.ci_upper,
        )
    }
}

/// In-memory store of eval run records.
#[derive(Debug, Default)]
pub struct EvalRunStore {
    records: Vec<EvalRunRecord>,
}

impl EvalRunStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a new record to the store.
    pub fn save(&mut self, record: EvalRunRecord) {
        self.records.push(record);
    }

    /// Retrieve a record by run ID.
    pub fn get(&self, run_id: &RunId) -> Option<&EvalRunRecord> {
        self.records.iter().find(|r| r.run_id == *run_id)
    }

    /// List all stored records.
    pub fn list(&self) -> &[EvalRunRecord] {
        &self.records
    }

    /// Serialize the store to a simple text format.
    pub fn serialize(&self) -> String {
        self.records
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Deserialize records from text (round-trip: only the string representation).
    /// Returns the raw lines; actual parsing would require field decoding.
    pub fn raw_lines(serialized: &str) -> Vec<String> {
        serialized
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.to_string())
            .collect()
    }

    /// Remove a record by run ID. Returns true if removed.
    pub fn remove(&mut self, run_id: &RunId) -> bool {
        let before = self.records.len();
        self.records.retain(|r| r.run_id != *run_id);
        self.records.len() < before
    }

    /// Count stored records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
