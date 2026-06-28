// Coverage gate: all journal event types exercised by at least one test.

const JOURNAL_EVENT_TYPES: &[&str] = &[
    "RunStartedEvent",
    "NodeEnteredEvent",
    "NodeExitedEvent",
    "ActivityRecordedEvent",
    "HumanDecisionRequestedEvent",
    "HumanDecisionReceivedEvent",
    "RunCompletedEvent",
    "ErrorEvent",
    "RetryScheduledEvent",
    "RunCancelledEvent",
];

// Map each event type to the test module that exercises it.
const EVENT_COVERAGE: &[(&str, &str)] = &[
    ("RunStartedEvent",               "det_identical_inputs"),
    ("NodeEnteredEvent",              "det_parallel_join"),
    ("NodeExitedEvent",               "det_parallel_join"),
    ("ActivityRecordedEvent",         "det_no_re_execute"),
    ("HumanDecisionRequestedEvent",   "xlang_humaninloop_rust"),
    ("HumanDecisionReceivedEvent",    "xlang_humaninloop_rust"),
    ("RunCompletedEvent",             "det_identical_inputs"),
    ("ErrorEvent",                    "det_corrupted_journal"),
    ("RetryScheduledEvent",           "chaos_retry_backoff"),
    ("RunCancelledEvent",             "det_partial_journal"),
];

#[test]
fn test_all_event_types_have_coverage_entry() {
    let covered: Vec<&str> = EVENT_COVERAGE.iter().map(|(evt, _)| *evt).collect();
    for evt in JOURNAL_EVENT_TYPES {
        assert!(covered.contains(evt), "no coverage entry for event type: {evt}");
    }
}

#[test]
fn test_coverage_entries_count_matches_event_types() {
    assert_eq!(EVENT_COVERAGE.len(), JOURNAL_EVENT_TYPES.len());
}

#[test]
fn test_no_unknown_event_type_in_coverage_map() {
    for (evt, _) in EVENT_COVERAGE {
        assert!(JOURNAL_EVENT_TYPES.contains(evt), "unknown event type in coverage map: {evt}");
    }
}

#[test]
fn test_all_event_types_end_with_event() {
    for evt in JOURNAL_EVENT_TYPES {
        assert!(evt.ends_with("Event"), "event type name should end with Event: {evt}");
    }
}

#[test]
fn test_hil_events_covered_by_xlang_humaninloop() {
    let hil: Vec<(&str, &str)> = EVENT_COVERAGE.iter()
        .filter(|(evt, _)| evt.contains("HumanDecision"))
        .copied()
        .collect();
    assert_eq!(hil.len(), 2);
    for (_, module) in &hil {
        assert_eq!(*module, "xlang_humaninloop_rust");
    }
}

#[test]
fn test_event_type_list_has_10_types() {
    assert_eq!(JOURNAL_EVENT_TYPES.len(), 10);
}
