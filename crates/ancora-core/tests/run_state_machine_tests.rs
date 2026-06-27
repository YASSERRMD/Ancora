use ancora_core::run::{Run, RunStatus};

#[test]
fn pending_to_running_is_legal() {
    let status = RunStatus::Pending;
    let next = status.transition(RunStatus::Running).unwrap();
    assert_eq!(next, RunStatus::Running);
}

#[test]
fn running_to_completed_is_legal() {
    let status = RunStatus::Running;
    let next = status.transition(RunStatus::Completed).unwrap();
    assert_eq!(next, RunStatus::Completed);
}

#[test]
fn running_to_cancelled_is_legal() {
    let status = RunStatus::Running;
    let next = status.transition(RunStatus::Cancelled).unwrap();
    assert_eq!(next, RunStatus::Cancelled);
}

#[test]
fn running_to_failed_is_legal() {
    let status = RunStatus::Running;
    let next = status.transition(RunStatus::Failed).unwrap();
    assert_eq!(next, RunStatus::Failed);
}

#[test]
fn pending_to_completed_is_illegal() {
    let err = RunStatus::Pending.transition(RunStatus::Completed);
    assert!(err.is_err(), "Pending -> Completed must be rejected");
}

#[test]
fn pending_to_failed_is_illegal() {
    let err = RunStatus::Pending.transition(RunStatus::Failed);
    assert!(err.is_err(), "Pending -> Failed must be rejected");
}

#[test]
fn pending_to_cancelled_is_illegal() {
    let err = RunStatus::Pending.transition(RunStatus::Cancelled);
    assert!(err.is_err(), "Pending -> Cancelled must be rejected");
}

#[test]
fn running_to_pending_is_illegal() {
    let err = RunStatus::Running.transition(RunStatus::Pending);
    assert!(err.is_err(), "Running -> Pending must be rejected");
}

#[test]
fn completed_is_terminal_no_further_transitions() {
    for next in [RunStatus::Running, RunStatus::Pending, RunStatus::Failed, RunStatus::Cancelled] {
        let err = RunStatus::Completed.transition(next);
        assert!(err.is_err(), "Completed must not transition to {:?}", next);
    }
}

#[test]
fn cancelled_is_terminal_no_further_transitions() {
    for next in [RunStatus::Running, RunStatus::Pending, RunStatus::Failed, RunStatus::Completed] {
        let err = RunStatus::Cancelled.transition(next);
        assert!(err.is_err(), "Cancelled must not transition to {:?}", next);
    }
}

#[test]
fn failed_is_terminal_no_further_transitions() {
    for next in [RunStatus::Running, RunStatus::Pending, RunStatus::Completed, RunStatus::Cancelled] {
        let err = RunStatus::Failed.transition(next);
        assert!(err.is_err(), "Failed must not transition to {:?}", next);
    }
}

#[test]
fn is_terminal_for_terminal_states() {
    assert!(RunStatus::Completed.is_terminal());
    assert!(RunStatus::Cancelled.is_terminal());
    assert!(RunStatus::Failed.is_terminal());
}

#[test]
fn is_not_terminal_for_non_terminal_states() {
    assert!(!RunStatus::Pending.is_terminal());
    assert!(!RunStatus::Running.is_terminal());
}

#[test]
fn run_generate_starts_in_pending() {
    let run = Run::generate();
    assert_eq!(run.status, RunStatus::Pending);
}

#[test]
fn run_new_uses_provided_id() {
    let run = Run::new("custom-id-42");
    assert_eq!(run.id, "custom-id-42");
}

#[test]
fn run_transition_mutates_status() {
    let mut run = Run::generate();
    run.transition(RunStatus::Running).unwrap();
    assert_eq!(run.status, RunStatus::Running);
    run.transition(RunStatus::Completed).unwrap();
    assert_eq!(run.status, RunStatus::Completed);
}

#[test]
fn run_generate_produces_unique_ids() {
    let ids: Vec<_> = (0..100).map(|_| Run::generate().id).collect();
    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(unique.len(), 100, "all generated IDs must be unique");
}
