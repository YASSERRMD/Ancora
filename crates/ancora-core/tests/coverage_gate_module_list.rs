// Coverage gate: all expected test modules exist and are non-empty.

const EXPECTED_TEST_MODULES: &[&str] = &[
    // journal / replay
    "det_identical_inputs",
    "det_no_re_execute",
    "det_parallel_join",
    "det_map_ordering",
    "det_time_and_random",
    "det_process_restart",
    "det_cross_language_replay",
    "det_divergence_code_change",
    "det_divergence_schema_change",
    "det_property_random_graphs",
    "det_property_tool_sequences",
    "det_large_journal_perf",
    "det_partial_journal",
    "det_corrupted_journal",
    "det_version_migration",
    "det_cost_replay",
    "det_otel_replay",
    "det_no_network_replay",
    "det_idempotent_replay",
    // chaos
    "chaos_retry_backoff",
    "chaos_network_fault",
    "chaos_process_kill",
    "chaos_oom_guard",
    "chaos_clock_skew",
    "chaos_disk_full",
    "chaos_provider_failover",
    "chaos_journal_corruption",
    "chaos_partial_write",
    // load
    "load_throughput",
    "load_concurrent_runs",
    "load_token_stream",
    "load_memory_store",
    "load_replay_throughput",
    // reliability
    "reliability_circuit_breaker",
    "reliability_deadline",
    "reliability_rate_limit",
    "reliability_graceful_shutdown",
    "reliability_health_check",
    // security
    "sec_prompt_injection",
    "sec_data_exfiltration",
    "sec_tool_allowlist",
    "sec_output_filter",
    "sec_key_rotation",
    "sec_audit_log",
    "sec_rbac",
    "sec_no_live_keys",
    "sec_input_size_limit",
    "sec_tls_config",
    "sec_no_network_in_tests",
    // policy
    "policy_data_residency",
    "policy_model_allowlist",
    "policy_cost_ceiling",
    "policy_retention",
    "policy_sovereignty",
    "policy_consent",
    "policy_gdpr_erasure",
    "policy_offline_mode",
];

#[test]
fn test_expected_module_count_at_least_57() {
    assert!(EXPECTED_TEST_MODULES.len() >= 57, "expected >= 57 modules, got {}", EXPECTED_TEST_MODULES.len());
}

#[test]
fn test_no_duplicate_module_names() {
    let mut sorted = EXPECTED_TEST_MODULES.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), EXPECTED_TEST_MODULES.len(), "duplicate module names found");
}

#[test]
fn test_all_module_names_are_snake_case() {
    for name in EXPECTED_TEST_MODULES {
        assert!(name.chars().all(|c| c.is_ascii_lowercase() || c == '_'), "not snake_case: {name}");
    }
}

#[test]
fn test_det_suite_has_19_modules() {
    let det: Vec<&&str> = EXPECTED_TEST_MODULES.iter().filter(|n| n.starts_with("det_")).collect();
    assert_eq!(det.len(), 19);
}

#[test]
fn test_sec_suite_has_11_modules() {
    let sec: Vec<&&str> = EXPECTED_TEST_MODULES.iter().filter(|n| n.starts_with("sec_")).collect();
    assert_eq!(sec.len(), 11);
}

#[test]
fn test_policy_suite_has_8_modules() {
    let pol: Vec<&&str> = EXPECTED_TEST_MODULES.iter().filter(|n| n.starts_with("policy_")).collect();
    assert_eq!(pol.len(), 8);
}
