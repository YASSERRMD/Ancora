use crate::lifecycle::{BackgroundRun, RunState};

#[test]
fn run_transitions_created_to_running() {
    let mut run = BackgroundRun::new("r1", 0);
    assert_eq!(run.state, RunState::Created);
    run.start();
    assert_eq!(run.state, RunState::Running);
}

#[test]
fn run_sleeps_and_wakes() {
    let mut run = BackgroundRun::new("r1", 0);
    run.start();
    run.sleep_until(100);
    assert!(!run.wake(50));
    assert!(run.wake(100));
    assert_eq!(run.state, RunState::Woken);
}

#[test]
fn run_completes() {
    let mut run = BackgroundRun::new("r1", 0);
    run.start();
    run.complete();
    assert_eq!(run.state, RunState::Completed);
}

#[test]
fn zero_duplicate_effects_across_wakeups() {
    let mut run = BackgroundRun::new("r1", 0);
    run.start();
    assert!(run.apply_effect("send-email"));
    assert!(!run.apply_effect("send-email"));
    assert_eq!(run.effects_applied.len(), 1);
}
