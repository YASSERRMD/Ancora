use std::collections::HashMap;

const POISON_THRESHOLD: usize = 5;

/// Tracks consecutive failure counts per run.
#[derive(Default)]
pub struct PoisonTracker {
    counts: HashMap<String, usize>,
    quarantined: std::collections::HashSet<String>,
}

impl PoisonTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_failure(&mut self, run_id: &str) -> bool {
        let count = self.counts.entry(run_id.to_string()).or_insert(0);
        *count += 1;
        if *count >= POISON_THRESHOLD {
            self.quarantined.insert(run_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn is_quarantined(&self, run_id: &str) -> bool {
        self.quarantined.contains(run_id)
    }

    pub fn reset(&mut self, run_id: &str) {
        self.counts.remove(run_id);
        self.quarantined.remove(run_id);
    }

    pub fn quarantined_set(&self) -> Vec<String> {
        self.quarantined.iter().cloned().collect()
    }
}
