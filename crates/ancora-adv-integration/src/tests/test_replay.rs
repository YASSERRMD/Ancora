use ancora_coord::CoordJournal;
use ancora_guard::GuardrailJournal;
use ancora_memcon::{ConsolidationEvent, ConsolidationJournal};
use ancora_reason::{ReasoningEvent, ReasoningJournal};

#[test]
fn all_combined_deterministic_replay() {
    // Each journal records its events; replay must return events in insertion order

    let mut coord_j = CoordJournal::default();
    coord_j.record(1, "blackboard", "agent-a wrote key=task");
    coord_j.record(2, "contract_net", "agent-b won bid");

    let mut guard_j = GuardrailJournal::default();
    // Record decisions by running a check
    use ancora_guard::{GuardrailPolicy, PiiInputGuardrail};
    let mut policy = GuardrailPolicy::new();
    policy.add_input(PiiInputGuardrail);
    policy.check_input("user@example.com data", &mut guard_j, 3);

    let mut reason_j = ReasoningJournal::default();
    reason_j.record(
        4,
        ReasoningEvent::StepAdded {
            index: 0,
            claim: "A".into(),
        },
    );
    reason_j.record(5, ReasoningEvent::StepVerified { index: 0 });

    let mut mem_j = ConsolidationJournal::default();
    mem_j.record(
        6,
        ConsolidationEvent::Summarized {
            dropped_count: 2,
            summary_len: 10,
        },
    );

    // Replay each journal deterministically
    let coord_replay = coord_j.events();
    assert_eq!(coord_replay.len(), 2);
    assert_eq!(coord_replay[0].tick, 1);

    let reason_replay = reason_j.replay();
    assert_eq!(reason_replay.len(), 2);
    assert_eq!(
        reason_replay[0],
        &ReasoningEvent::StepAdded {
            index: 0,
            claim: "A".into()
        }
    );

    let mem_entries = mem_j.entries();
    assert_eq!(mem_entries.len(), 1);
}
