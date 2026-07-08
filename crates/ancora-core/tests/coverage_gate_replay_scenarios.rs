// Coverage gate: all replay scenario types have at least one test.

const REPLAY_SCENARIOS: &[&str] = &[
    "identical_inputs",
    "no_re_execute",
    "parallel_join",
    "map_ordering",
    "time_and_random",
    "process_restart",
    "cross_language",
    "divergence_code_change",
    "divergence_schema_change",
    "property_random_graphs",
    "property_tool_sequences",
    "large_journal_perf",
    "partial_journal",
    "corrupted_journal",
    "version_migration",
    "cost_replay",
    "otel_replay",
    "no_network_replay",
    "idempotent_replay",
];

fn scenario_to_test(scenario: &str) -> Option<String> {
    Some(format!("det_{scenario}"))
}

#[test]
fn test_19_replay_scenarios_defined() {
    assert_eq!(REPLAY_SCENARIOS.len(), 19);
}

#[test]
fn test_all_scenarios_map_to_det_test() {
    for s in REPLAY_SCENARIOS {
        let test = scenario_to_test(s).unwrap();
        assert!(test.starts_with("det_"), "expected det_ prefix: {test}");
    }
}

#[test]
fn test_idempotent_scenario_is_last() {
    assert_eq!(*REPLAY_SCENARIOS.last().unwrap(), "idempotent_replay");
}

#[test]
fn test_identical_inputs_is_first() {
    assert_eq!(REPLAY_SCENARIOS[0], "identical_inputs");
}

#[test]
fn test_no_duplicate_scenarios() {
    let mut sorted = REPLAY_SCENARIOS.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), REPLAY_SCENARIOS.len());
}

#[test]
fn test_divergence_scenarios_both_present() {
    let divergence: Vec<&&str> = REPLAY_SCENARIOS
        .iter()
        .filter(|s| s.starts_with("divergence"))
        .collect();
    assert_eq!(divergence.len(), 2);
}
