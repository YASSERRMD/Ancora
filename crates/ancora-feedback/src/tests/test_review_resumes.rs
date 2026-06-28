use crate::decision::{DecisionOutcome, DecisionStore, ReviewDecision};
use crate::queue::ReviewQueue;

/// A simple run handle that can be paused and resumed.
#[derive(Debug, PartialEq, Eq)]
enum RunState {
    Running,
    Paused,
    Resumed,
    Rejected,
}

struct RunHandle {
    run_id: String,
    state: RunState,
}

impl RunHandle {
    fn new(run_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            state: RunState::Running,
        }
    }

    fn pause(&mut self) {
        self.state = RunState::Paused;
    }

    /// Apply a review decision to the run. Returns true if the run was resumed.
    fn apply_decision(&mut self, outcome: &DecisionOutcome) -> bool {
        match outcome {
            DecisionOutcome::Approve => {
                self.state = RunState::Resumed;
                true
            }
            DecisionOutcome::Reject => {
                self.state = RunState::Rejected;
                false
            }
            DecisionOutcome::RequestChanges => false,
        }
    }
}

#[test]
fn review_resumes_a_paused_run() {
    let mut queue = ReviewQueue::with_threshold(0.5);
    let mut run = RunHandle::new("run-paused");

    // Low confidence: queue and pause
    queue.submit(&run.run_id, 0.2);
    run.pause();
    assert_eq!(run.state, RunState::Paused);

    // Reviewer claims and approves
    queue.claim(&run.run_id);

    let mut decisions = DecisionStore::new();
    decisions.record(ReviewDecision {
        run_id: run.run_id.clone(),
        reviewer_id: "r1".into(),
        outcome: DecisionOutcome::Approve,
        notes: None,
        decided_at: 1000,
    });

    // Apply the decision to resume the run
    let decision = decisions.latest_for_run(&run.run_id).unwrap();
    let resumed = run.apply_decision(&decision.outcome);
    assert!(resumed);
    assert_eq!(run.state, RunState::Resumed);
}

#[test]
fn reject_decision_does_not_resume_run() {
    let mut run = RunHandle::new("run-rejected");
    run.pause();

    let mut decisions = DecisionStore::new();
    decisions.record(ReviewDecision {
        run_id: run.run_id.clone(),
        reviewer_id: "r1".into(),
        outcome: DecisionOutcome::Reject,
        notes: Some("Unsafe output".into()),
        decided_at: 2000,
    });

    let decision = decisions.latest_for_run(&run.run_id).unwrap();
    let resumed = run.apply_decision(&decision.outcome);
    assert!(!resumed);
    assert_eq!(run.state, RunState::Rejected);
}

#[test]
fn request_changes_leaves_run_paused() {
    let mut run = RunHandle::new("run-pending");
    run.pause();

    let mut decisions = DecisionStore::new();
    decisions.record(ReviewDecision {
        run_id: run.run_id.clone(),
        reviewer_id: "r2".into(),
        outcome: DecisionOutcome::RequestChanges,
        notes: Some("Needs revision".into()),
        decided_at: 3000,
    });

    let decision = decisions.latest_for_run(&run.run_id).unwrap();
    let resumed = run.apply_decision(&decision.outcome);
    assert!(!resumed);
    assert_eq!(run.state, RunState::Paused);
}
