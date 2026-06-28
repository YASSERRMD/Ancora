use std::collections::HashSet;

/// Tracks in-flight run IDs to reconcile after failover.
#[derive(Default)]
pub struct RunTracker {
    in_flight: HashSet<String>,
    completed: HashSet<String>,
}

impl RunTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, run_id: impl Into<String>) {
        self.in_flight.insert(run_id.into());
    }

    pub fn complete(&mut self, run_id: &str) {
        self.in_flight.remove(run_id);
        self.completed.insert(run_id.to_owned());
    }

    pub fn in_flight(&self) -> &HashSet<String> {
        &self.in_flight
    }

    pub fn is_completed(&self, run_id: &str) -> bool {
        self.completed.contains(run_id)
    }

    /// Returns run IDs that must be resumed after failover.
    pub fn runs_to_resume(&self) -> Vec<String> {
        self.in_flight.iter().cloned().collect()
    }
}
