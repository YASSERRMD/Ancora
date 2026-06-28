/// Records that a subject was exposed to a variant during an experiment.

use std::time::{SystemTime, UNIX_EPOCH};

/// An exposure event: records when a subject saw a specific variant.
#[derive(Debug, Clone, PartialEq)]
pub struct Exposure {
    pub experiment_id: String,
    pub subject_key: String,
    pub variant_name: String,
    /// Unix timestamp in seconds when the exposure occurred.
    pub timestamp_secs: u64,
}

impl Exposure {
    /// Create an exposure stamped with the current system time.
    pub fn now(
        experiment_id: impl Into<String>,
        subject_key: impl Into<String>,
        variant_name: impl Into<String>,
    ) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Exposure {
            experiment_id: experiment_id.into(),
            subject_key: subject_key.into(),
            variant_name: variant_name.into(),
            timestamp_secs: ts,
        }
    }

    /// Create an exposure with an explicit timestamp (useful in tests).
    pub fn with_timestamp(
        experiment_id: impl Into<String>,
        subject_key: impl Into<String>,
        variant_name: impl Into<String>,
        timestamp_secs: u64,
    ) -> Self {
        Exposure {
            experiment_id: experiment_id.into(),
            subject_key: subject_key.into(),
            variant_name: variant_name.into(),
            timestamp_secs,
        }
    }
}

/// An in-memory log of exposure events.
#[derive(Debug, Default)]
pub struct ExposureLog {
    entries: Vec<Exposure>,
}

impl ExposureLog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append an exposure to the log.
    pub fn record(&mut self, exposure: Exposure) {
        self.entries.push(exposure);
    }

    /// Return all recorded exposures.
    pub fn all(&self) -> &[Exposure] {
        &self.entries
    }

    /// Return exposures for a specific experiment.
    pub fn for_experiment<'a>(&'a self, experiment_id: &str) -> impl Iterator<Item = &'a Exposure> {
        let exp_id = experiment_id.to_string();
        self.entries.iter().filter(move |e| e.experiment_id == exp_id)
    }

    /// Return exposures for a specific variant within an experiment.
    pub fn for_variant<'a>(
        &'a self,
        experiment_id: &str,
        variant_name: &str,
    ) -> impl Iterator<Item = &'a Exposure> {
        let exp_id = experiment_id.to_string();
        let var = variant_name.to_string();
        self.entries
            .iter()
            .filter(move |e| e.experiment_id == exp_id && e.variant_name == var)
    }

    /// Count unique subjects exposed to a variant.
    pub fn unique_subjects_for_variant(&self, experiment_id: &str, variant_name: &str) -> usize {
        let mut seen = std::collections::HashSet::new();
        for e in self.for_variant(experiment_id, variant_name) {
            seen.insert(e.subject_key.clone());
        }
        seen.len()
    }
}
