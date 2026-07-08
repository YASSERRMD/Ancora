use crate::attack::{AttackLog, AttackOutcome, AttackStep, AttackVector};

fn step(id: &str, scenario_id: &str, vector: AttackVector, outcome: AttackOutcome) -> AttackStep {
    AttackStep::new(id, scenario_id, "N", vector, outcome, "", "", 1)
}

#[test]
fn for_scenario_filters() {
    let mut log = AttackLog::new();
    log.record(step(
        "s1",
        "sc1",
        AttackVector::Network,
        AttackOutcome::Success,
    ));
    log.record(step(
        "s2",
        "sc2",
        AttackVector::Local,
        AttackOutcome::Failure,
    ));
    assert_eq!(log.for_scenario("sc1").len(), 1);
    assert_eq!(log.for_scenario("sc2").len(), 1);
    assert_eq!(log.for_scenario("none").len(), 0);
}

#[test]
fn successful_filters() {
    let mut log = AttackLog::new();
    log.record(step(
        "s1",
        "sc1",
        AttackVector::Network,
        AttackOutcome::Success,
    ));
    log.record(step(
        "s2",
        "sc1",
        AttackVector::Network,
        AttackOutcome::PartialSuccess,
    ));
    log.record(step(
        "s3",
        "sc1",
        AttackVector::Network,
        AttackOutcome::Failure,
    ));
    assert_eq!(log.successful().len(), 2);
}

#[test]
fn detected_filters() {
    let mut log = AttackLog::new();
    log.record(step(
        "s1",
        "sc1",
        AttackVector::Network,
        AttackOutcome::Detected,
    ));
    log.record(step(
        "s2",
        "sc1",
        AttackVector::Network,
        AttackOutcome::Success,
    ));
    assert_eq!(log.detected().len(), 1);
}

#[test]
fn by_vector_filters() {
    let mut log = AttackLog::new();
    log.record(step(
        "s1",
        "sc1",
        AttackVector::Network,
        AttackOutcome::Success,
    ));
    log.record(step(
        "s2",
        "sc1",
        AttackVector::Local,
        AttackOutcome::Success,
    ));
    assert_eq!(log.by_vector(&AttackVector::Network).len(), 1);
    assert_eq!(log.by_vector(&AttackVector::Physical).len(), 0);
}
