/// Automatic dataset refresh for continuous evaluation.
///
/// Periodically rotates the evaluation dataset so that stale samples
/// are replaced with fresh production traffic, keeping the eval set
/// representative of current usage.

use std::time::{Duration, SystemTime};

/// Policy that controls how frequently the dataset is refreshed.
#[derive(Debug, Clone)]
pub struct RefreshPolicy {
    /// Maximum age of a sample before it is considered stale.
    pub max_sample_age: Duration,
    /// Minimum number of fresh samples required before a refresh is triggered.
    pub min_fresh_samples: usize,
    /// Maximum size of the dataset after a refresh.
    pub max_dataset_size: usize,
}

impl RefreshPolicy {
    pub fn new(max_age_secs: u64, min_fresh: usize, max_size: usize) -> Self {
        RefreshPolicy {
            max_sample_age: Duration::from_secs(max_age_secs),
            min_fresh_samples: min_fresh,
            max_dataset_size: max_size,
        }
    }
}

/// A dataset entry with an ingestion timestamp.
#[derive(Debug, Clone)]
pub struct DatasetEntry {
    pub id: String,
    pub ingested_at: SystemTime,
    pub payload: String,
}

impl DatasetEntry {
    pub fn new(id: impl Into<String>, ingested_at: SystemTime, payload: impl Into<String>) -> Self {
        DatasetEntry {
            id: id.into(),
            ingested_at,
            payload: payload.into(),
        }
    }

    /// Returns true if the entry is older than the given max age.
    pub fn is_stale(&self, now: SystemTime, max_age: Duration) -> bool {
        now.duration_since(self.ingested_at)
            .map(|age| age > max_age)
            .unwrap_or(false)
    }
}

/// Dataset manager that applies the refresh policy.
#[derive(Debug)]
pub struct DatasetRefresher {
    policy: RefreshPolicy,
    entries: Vec<DatasetEntry>,
    last_refresh: Option<SystemTime>,
}

impl DatasetRefresher {
    pub fn new(policy: RefreshPolicy) -> Self {
        DatasetRefresher {
            policy,
            entries: Vec::new(),
            last_refresh: None,
        }
    }

    /// Ingest a new entry into the dataset.
    pub fn ingest(&mut self, entry: DatasetEntry) {
        self.entries.push(entry);
    }

    /// Remove stale entries given the current time. Returns the number
    /// of entries removed.
    pub fn evict_stale(&mut self, now: SystemTime) -> usize {
        let max_age = self.policy.max_sample_age;
        let before = self.entries.len();
        self.entries.retain(|e| !e.is_stale(now, max_age));
        before - self.entries.len()
    }

    /// Check whether a refresh is needed:
    /// - There are fewer fresh samples than the minimum, OR
    /// - The dataset exceeds the maximum size (triggering a trim).
    pub fn needs_refresh(&self, now: SystemTime) -> bool {
        let fresh = self
            .entries
            .iter()
            .filter(|e| !e.is_stale(now, self.policy.max_sample_age))
            .count();
        fresh < self.policy.min_fresh_samples
    }

    /// Perform a refresh: evict stale entries and trim to max size.
    /// Returns the count of entries removed.
    pub fn refresh(&mut self, now: SystemTime) -> usize {
        let removed = self.evict_stale(now);
        // Trim to max size, keeping newest entries.
        let max = self.policy.max_dataset_size;
        if self.entries.len() > max {
            let drain_count = self.entries.len() - max;
            self.entries.drain(0..drain_count);
        }
        self.last_refresh = Some(now);
        removed
    }

    /// Number of entries currently in the dataset.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the dataset is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Timestamp of the last refresh, if any.
    pub fn last_refresh(&self) -> Option<SystemTime> {
        self.last_refresh
    }
}
