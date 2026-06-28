use crate::escalation::{EscalationChannel, EscalationLevel, EscalationPolicy};
use crate::incident::Severity;
use crate::runbook::{Runbook, RunbookStep};

pub fn security_runbook(incident_id: impl Into<String>) -> Runbook {
    let mut rb = Runbook::new("rb-security", "Security Incident Runbook", incident_id);
    rb.add_step(RunbookStep::new("s1", "Isolate affected systems", "Revoke access tokens and network isolation"));
    rb.add_step(RunbookStep::new("s2", "Gather evidence", "Collect logs, screenshots, and artifacts"));
    rb.add_step(RunbookStep::new("s3", "Notify stakeholders", "Alert security team and management"));
    rb.add_step(RunbookStep::new("s4", "Apply remediation", "Deploy patches or configuration fixes"));
    rb.add_step(RunbookStep::new("s5", "Verify fix", "Confirm systems are clean and monitoring"));
    rb
}

pub fn critical_escalation_policy(tenant_id: impl Into<String>) -> EscalationPolicy {
    let mut policy = EscalationPolicy::new(tenant_id, Severity::High);
    policy.add_level(EscalationLevel::new(1, "on-call-engineer", EscalationChannel::Pager, 0));
    policy.add_level(EscalationLevel::new(2, "team-lead", EscalationChannel::Phone, 300));
    policy.add_level(EscalationLevel::new(3, "vp-engineering", EscalationChannel::Phone, 900));
    policy
}
