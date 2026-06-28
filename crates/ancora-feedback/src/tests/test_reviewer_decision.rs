use crate::decision::{DecisionOutcome, DecisionStore, ReviewDecision};
use crate::reviewer::{Reviewer, ReviewerRegistry};

#[test]
fn reviewer_decision_recorded() {
    let mut store = DecisionStore::new();
    store.record(ReviewDecision {
        run_id: "run-1".into(),
        reviewer_id: "rev-alice".into(),
        outcome: DecisionOutcome::Approve,
        notes: Some("All good".into()),
        decided_at: 12345,
    });

    let decision = store.latest_for_run("run-1").unwrap();
    assert_eq!(decision.reviewer_id, "rev-alice");
    assert_eq!(decision.outcome, DecisionOutcome::Approve);
    assert_eq!(decision.notes.as_deref(), Some("All good"));
}

#[test]
fn latest_decision_overrides_earlier() {
    let mut store = DecisionStore::new();
    store.record(ReviewDecision {
        run_id: "run-2".into(),
        reviewer_id: "r1".into(),
        outcome: DecisionOutcome::RequestChanges,
        notes: None,
        decided_at: 1,
    });
    store.record(ReviewDecision {
        run_id: "run-2".into(),
        reviewer_id: "r2".into(),
        outcome: DecisionOutcome::Approve,
        notes: None,
        decided_at: 2,
    });

    let latest = store.latest_for_run("run-2").unwrap();
    assert_eq!(latest.outcome, DecisionOutcome::Approve);
    assert_eq!(latest.reviewer_id, "r2");
}

#[test]
fn reviewer_assigned_and_decision_matches() {
    let mut reg = ReviewerRegistry::new();
    reg.register(Reviewer {
        id: "r-bob".into(),
        name: "Bob".into(),
        email: "bob@example.com".into(),
    });
    reg.assign("run-3", "r-bob").unwrap();

    let mut store = DecisionStore::new();
    store.record(ReviewDecision {
        run_id: "run-3".into(),
        reviewer_id: "r-bob".into(),
        outcome: DecisionOutcome::Reject,
        notes: Some("Hallucinated facts".into()),
        decided_at: 999,
    });

    let rev = reg.assigned_reviewer("run-3").unwrap();
    let dec = store.latest_for_run("run-3").unwrap();
    assert_eq!(rev.id, dec.reviewer_id);
    assert_eq!(dec.outcome, DecisionOutcome::Reject);
}

#[test]
fn count_by_outcome() {
    let mut store = DecisionStore::new();
    for i in 0..3 {
        store.record(ReviewDecision {
            run_id: format!("run-{}", i),
            reviewer_id: "r1".into(),
            outcome: DecisionOutcome::Approve,
            notes: None,
            decided_at: i as u64,
        });
    }
    store.record(ReviewDecision {
        run_id: "run-x".into(),
        reviewer_id: "r1".into(),
        outcome: DecisionOutcome::Reject,
        notes: None,
        decided_at: 100,
    });

    assert_eq!(store.count_by_outcome(&DecisionOutcome::Approve), 3);
    assert_eq!(store.count_by_outcome(&DecisionOutcome::Reject), 1);
}
