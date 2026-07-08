use crate::attack::{AttackOutcome, AttackStep, AttackVector};

#[test]
fn basic_fields() {
    let step = AttackStep::new(
        "s1",
        "sc1",
        "Port Scan",
        AttackVector::Network,
        AttackOutcome::Success,
        "T1046",
        "Open ports found",
        5,
    );
    assert_eq!(step.id, "s1");
    assert_eq!(step.scenario_id, "sc1");
    assert_eq!(step.name, "Port Scan");
    assert_eq!(step.vector, AttackVector::Network);
    assert_eq!(step.outcome, AttackOutcome::Success);
    assert_eq!(step.technique, "T1046");
    assert_eq!(step.detail, "Open ports found");
    assert_eq!(step.tick, 5);
}
