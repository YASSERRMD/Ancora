use ancora_coord::CoordJournal;
use ancora_guard::{GuardrailJournal, GuardrailPolicy, SafetyOutputGuardrail};
use ancora_memcon::{ConsolidationEvent, ConsolidationJournal};
use ancora_reason::{ReasoningEvent, ReasoningJournal};
use ancora_toolsynth::{AuditEvent, SynthAudit};

#[test]
fn combined_audit_complete() {
    // Every subsystem journals its events; combined audit trail is non-empty

    // Toolsynth audit
    let mut synth_audit = SynthAudit::default();
    synth_audit.record(
        1,
        AuditEvent::Synthesized {
            tool_name: "search_docs".into(),
            goal: "search documents".into(),
        },
    );
    synth_audit.record(
        2,
        AuditEvent::Approved {
            tool_name: "search_docs".into(),
            approver: "admin".into(),
        },
    );
    assert_eq!(synth_audit.entries().len(), 2);

    // Guard journal
    let mut guard_j = GuardrailJournal::default();
    let mut policy = GuardrailPolicy::new();
    policy.add_output(SafetyOutputGuardrail);
    policy.check_output("<script>xss</script>", &mut guard_j, 1);
    assert_eq!(guard_j.blocked_count(), 1);

    // Coord journal
    let mut coord_j = CoordJournal::default();
    coord_j.record(2, "blackboard", "agent wrote key=audit-done");
    assert!(!coord_j.events().is_empty());

    // Reason journal
    let mut reason_j = ReasoningJournal::default();
    reason_j.record(
        3,
        ReasoningEvent::StepAdded {
            index: 0,
            claim: "audit claim".into(),
        },
    );
    assert!(!reason_j.events().is_empty());

    // Consolidation journal
    let mut mem_j = ConsolidationJournal::default();
    mem_j.record(4, ConsolidationEvent::Deduped { removed_count: 1 });
    assert_eq!(mem_j.entries().len(), 1);
}
