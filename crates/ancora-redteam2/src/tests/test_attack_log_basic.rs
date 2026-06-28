use crate::attack::{AttackLog, AttackOutcome, AttackStep, AttackVector};

fn step(id: &str, scenario_id: &str, outcome: AttackOutcome) -> AttackStep {
    AttackStep::new(id, scenario_id, "N", AttackVector::Network, outcome, "", "", 1)
}

#[test]
fn empty_log() {
    let log = AttackLog::new();
    assert_eq!(log.count(), 0);
}

#[test]
fn record_and_count() {
    let mut log = AttackLog::new();
    log.record(step("s1", "sc1", AttackOutcome::Success));
    log.record(step("s2", "sc1", AttackOutcome::Failure));
    assert_eq!(log.count(), 2);
}

#[test]
fn all_iterator() {
    let mut log = AttackLog::new();
    log.record(step("s1", "sc1", AttackOutcome::Success));
    assert_eq!(log.all().count(), 1);
}
