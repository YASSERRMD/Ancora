/// The outcome a reviewer chooses for a queued run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionOutcome {
    /// The reviewer approves the run output as correct.
    Approve,
    /// The reviewer rejects the run output.
    Reject,
    /// The reviewer requests changes before the run can be released.
    RequestChanges,
}

/// A captured review decision.
#[derive(Debug, Clone)]
pub struct ReviewDecision {
    pub run_id: String,
    pub reviewer_id: String,
    pub outcome: DecisionOutcome,
    /// Optional notes from the reviewer.
    pub notes: Option<String>,
    /// Unix timestamp when the decision was made.
    pub decided_at: u64,
}

/// Store for review decisions.
#[derive(Debug, Default)]
pub struct DecisionStore {
    decisions: Vec<ReviewDecision>,
}

impl DecisionStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a decision.
    pub fn record(&mut self, decision: ReviewDecision) {
        self.decisions.push(decision);
    }

    /// Find the latest decision for a run.
    pub fn latest_for_run(&self, run_id: &str) -> Option<&ReviewDecision> {
        self.decisions.iter().rev().find(|d| d.run_id == run_id)
    }

    /// Return all decisions.
    pub fn all(&self) -> &[ReviewDecision] {
        &self.decisions
    }

    /// Count decisions by outcome.
    pub fn count_by_outcome(&self, outcome: &DecisionOutcome) -> usize {
        self.decisions
            .iter()
            .filter(|d| &d.outcome == outcome)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_retrieve_decision() {
        let mut store = DecisionStore::new();
        store.record(ReviewDecision {
            run_id: "run-1".into(),
            reviewer_id: "r1".into(),
            outcome: DecisionOutcome::Approve,
            notes: None,
            decided_at: 100,
        });
        let d = store.latest_for_run("run-1").unwrap();
        assert_eq!(d.outcome, DecisionOutcome::Approve);
    }
}
