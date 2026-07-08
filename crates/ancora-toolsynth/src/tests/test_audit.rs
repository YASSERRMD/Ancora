use crate::audit::{AuditEvent, SynthAudit};

#[test]
fn audit_records_synthesis() {
    let mut audit = SynthAudit::default();
    audit.record(
        1,
        AuditEvent::Synthesized {
            tool_name: "t".into(),
            goal: "g".into(),
        },
    );
    assert_eq!(audit.entries().len(), 1);
}

#[test]
fn events_for_tool_filters_by_name() {
    let mut audit = SynthAudit::default();
    audit.record(
        1,
        AuditEvent::Synthesized {
            tool_name: "a".into(),
            goal: "ga".into(),
        },
    );
    audit.record(
        2,
        AuditEvent::Approved {
            tool_name: "a".into(),
            approver: "op".into(),
        },
    );
    audit.record(
        3,
        AuditEvent::Executed {
            tool_name: "b".into(),
        },
    );
    assert_eq!(audit.events_for_tool("a").len(), 2);
    assert_eq!(audit.events_for_tool("b").len(), 1);
}

#[test]
fn audit_trail_records_all_event_types() {
    let mut audit = SynthAudit::default();
    audit.record(
        1,
        AuditEvent::Cached {
            tool_name: "c".into(),
        },
    );
    audit.record(
        2,
        AuditEvent::Revoked {
            tool_name: "c".into(),
        },
    );
    assert_eq!(audit.events_for_tool("c").len(), 2);
}
