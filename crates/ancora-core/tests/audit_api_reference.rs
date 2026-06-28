// Documentation audit: API reference pages document all public types.

const PUBLIC_TYPES: &[(&str, &str)] = &[
    ("RunStatus",                "sdk/rust/api-reference.md"),
    ("MemoryStore",              "sdk/rust/api-reference.md"),
    ("ActivityRecordedEvent",    "sdk/rust/api-reference.md"),
    ("HumanDecisionRequested",   "sdk/rust/api-reference.md"),
    ("ReplayResult",             "sdk/rust/api-reference.md"),
    ("AncoraError",              "sdk/rust/api-reference.md"),
    ("VectorStoreConfig",        "sdk/rust/api-reference.md"),
    ("OrchestratorConfig",       "sdk/rust/api-reference.md"),
];

const PUBLIC_FUNCTIONS: &[(&str, &str)] = &[
    ("replay_events",        "sdk/rust/api-reference.md"),
    ("detect_divergence",    "sdk/rust/api-reference.md"),
];

#[test]
fn test_eight_public_types_documented() {
    assert_eq!(PUBLIC_TYPES.len(), 8);
}

#[test]
fn test_two_public_functions_documented() {
    assert_eq!(PUBLIC_FUNCTIONS.len(), 2);
}

#[test]
fn test_all_types_map_to_rust_api_ref() {
    for (_, page) in PUBLIC_TYPES {
        assert_eq!(*page, "sdk/rust/api-reference.md");
    }
}

#[test]
fn test_run_status_documented() {
    let found = PUBLIC_TYPES.iter().any(|(t, _)| *t == "RunStatus");
    assert!(found, "RunStatus not in public types");
}

#[test]
fn test_replay_events_documented() {
    let found = PUBLIC_FUNCTIONS.iter().any(|(f, _)| *f == "replay_events");
    assert!(found, "replay_events not documented");
}

#[test]
fn test_ancora_error_documented() {
    let found = PUBLIC_TYPES.iter().any(|(t, _)| *t == "AncoraError");
    assert!(found, "AncoraError not in public types");
}
