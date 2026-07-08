use crate::schema::Feedback;
use std::collections::HashMap;

/// Store that attaches feedback records to runs and steps.
#[derive(Debug, Default)]
pub struct FeedbackStore {
    /// Map from run_id to list of Feedback records.
    records: HashMap<String, Vec<Feedback>>,
}

impl FeedbackStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a feedback record to the store.
    pub fn attach(&mut self, feedback: Feedback) {
        self.records
            .entry(feedback.run_id.clone())
            .or_default()
            .push(feedback);
    }

    /// Retrieve all feedback for a given run.
    pub fn for_run(&self, run_id: &str) -> &[Feedback] {
        self.records.get(run_id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Retrieve feedback for a specific step within a run.
    pub fn for_step<'a>(&'a self, run_id: &str, step_id: &str) -> Vec<&'a Feedback> {
        self.for_run(run_id)
            .iter()
            .filter(|f| f.step_id.as_deref() == Some(step_id))
            .collect()
    }

    /// Total number of feedback records across all runs.
    pub fn total(&self) -> usize {
        self.records.values().map(Vec::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Feedback, ThumbsRating};

    #[test]
    fn attach_and_retrieve() {
        let mut store = FeedbackStore::new();
        let fb = Feedback::new("f1", "run-1", None, ThumbsRating::Up, None, "alice", 0);
        store.attach(fb);
        assert_eq!(store.for_run("run-1").len(), 1);
        assert_eq!(store.total(), 1);
    }
}
