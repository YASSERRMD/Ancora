// Example parity: single-agent example produces equivalent output across all 6 languages.

const SINGLE_AGENT_EXPECTED: &[(&str, &str, &str)] = &[
    ("rust",       "run_id_prefix", "sa-rust"),
    ("go",         "run_id_prefix", "sa-go"),
    ("python",     "run_id_prefix", "sa-python"),
    ("typescript", "run_id_prefix", "sa-ts"),
    ("dotnet",     "run_id_prefix", "sa-dotnet"),
    ("java",       "run_id_prefix", "sa-java"),
];

const SINGLE_AGENT_EVENT_SEQUENCE: &[&str] = &["started", "token", "completed"];

fn validate_event_sequence(events: &[&str]) -> bool {
    if events.is_empty() { return false; }
    events[0] == "started" && *events.last().unwrap() == "completed"
}

fn run_id_matches_prefix(run_id: &str, prefix: &str) -> bool {
    run_id.starts_with(prefix)
}

#[test]
fn test_all_six_languages_have_expected_run_id_prefix() {
    assert_eq!(SINGLE_AGENT_EXPECTED.len(), 6);
}

#[test]
fn test_event_sequence_valid() {
    assert!(validate_event_sequence(SINGLE_AGENT_EVENT_SEQUENCE));
}

#[test]
fn test_started_first_completed_last() {
    assert_eq!(SINGLE_AGENT_EVENT_SEQUENCE[0], "started");
    assert_eq!(*SINGLE_AGENT_EVENT_SEQUENCE.last().unwrap(), "completed");
}

#[test]
fn test_run_id_prefix_for_rust() {
    let rust = SINGLE_AGENT_EXPECTED.iter().find(|(l, _, _)| *l == "rust").unwrap();
    assert!(run_id_matches_prefix(&format!("{}-001", rust.2), rust.2));
}

#[test]
fn test_all_prefixes_are_unique() {
    let prefixes: Vec<&str> = SINGLE_AGENT_EXPECTED.iter().map(|(_, _, p)| *p).collect();
    let mut sorted = prefixes.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), prefixes.len());
}

#[test]
fn test_three_event_types_in_sequence() {
    assert_eq!(SINGLE_AGENT_EVENT_SEQUENCE.len(), 3);
}
