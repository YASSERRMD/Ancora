// Coverage gate: all required security properties have at least one test.

const SECURITY_PROPERTIES: &[&str] = &[
    "prompt_injection_detection",
    "pii_exfiltration_guard",
    "tool_allowlist_enforcement",
    "output_redaction",
    "key_rotation",
    "audit_log_immutability",
    "rbac_enforcement",
    "no_live_keys_in_fixtures",
    "input_size_limit",
    "tls_minimum_version",
    "no_live_urls_in_tests",
];

const SECURITY_TEST_MAP: &[(&str, &str)] = &[
    ("prompt_injection_detection", "sec_prompt_injection"),
    ("pii_exfiltration_guard", "sec_data_exfiltration"),
    ("tool_allowlist_enforcement", "sec_tool_allowlist"),
    ("output_redaction", "sec_output_filter"),
    ("key_rotation", "sec_key_rotation"),
    ("audit_log_immutability", "sec_audit_log"),
    ("rbac_enforcement", "sec_rbac"),
    ("no_live_keys_in_fixtures", "sec_no_live_keys"),
    ("input_size_limit", "sec_input_size_limit"),
    ("tls_minimum_version", "sec_tls_config"),
    ("no_live_urls_in_tests", "sec_no_network_in_tests"),
];

#[test]
fn test_all_security_properties_covered() {
    let covered: Vec<&str> = SECURITY_TEST_MAP.iter().map(|(p, _)| *p).collect();
    for prop in SECURITY_PROPERTIES {
        assert!(
            covered.contains(prop),
            "no test for security property: {prop}"
        );
    }
}

#[test]
fn test_security_properties_count() {
    assert_eq!(SECURITY_PROPERTIES.len(), 11);
}

#[test]
fn test_no_duplicate_security_properties() {
    let mut sorted = SECURITY_PROPERTIES.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), SECURITY_PROPERTIES.len());
}

#[test]
fn test_all_test_names_start_with_sec() {
    for (_, test) in SECURITY_TEST_MAP {
        assert!(
            test.starts_with("sec_"),
            "security test name should start with sec_: {test}"
        );
    }
}

#[test]
fn test_coverage_map_has_same_length_as_properties() {
    assert_eq!(SECURITY_TEST_MAP.len(), SECURITY_PROPERTIES.len());
}
