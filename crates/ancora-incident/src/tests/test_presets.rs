use crate::incident::Severity;
use crate::presets::{critical_escalation_policy, security_runbook};

#[test]
fn security_runbook_has_steps() {
    let rb = security_runbook("i1");
    assert!(rb.step_count() >= 5);
    assert_eq!(rb.incident_id, "i1");
}

#[test]
fn critical_escalation_has_levels() {
    let policy = critical_escalation_policy("t1");
    assert!(policy.level_count() >= 3);
    assert!(policy.should_escalate(&Severity::Critical));
}
