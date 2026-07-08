// Security: verify that test fixture JSON blobs contain no http/https URLs
// that would indicate live network calls in test data.

const FIXTURE_PAYLOADS: &[(&str, &str)] = &[
    (
        "sec_prompt_injection",
        r#"{"input":"what is the weather today?"}"#,
    ),
    (
        "sec_data_exfiltration",
        r#"{"query":"tell me about Paris"}"#,
    ),
    (
        "sec_tool_allowlist",
        r#"{"tool":"web_search","args":{"q":"rust lang"}}"#,
    ),
    (
        "sec_output_filter",
        r#"{"model":"claude-3-5-haiku","max_tokens":1024}"#,
    ),
    ("sec_key_rotation", r#"{"key_id":2,"encrypted":12345}"#),
    (
        "sec_audit_log",
        r#"{"op":"tool_call","actor":"user-1","allowed":true}"#,
    ),
    ("sec_rbac", r#"{"role":"operator","action":"execute"}"#),
    (
        "policy_cost_ceiling",
        r#"{"cost_usd":0.30,"model":"claude-3-5-haiku"}"#,
    ),
    (
        "policy_retention",
        r#"{"run_id":"test-001","created_at_ns":1700000000000000000}"#,
    ),
    (
        "policy_sovereignty",
        r#"{"target":"local","model":"qwen3"}"#,
    ),
];

fn contains_live_url(payload: &str) -> bool {
    payload.contains("https://") || payload.contains("http://api.")
}

#[test]
fn test_all_fixtures_are_offline() {
    for (name, payload) in FIXTURE_PAYLOADS {
        assert!(
            !contains_live_url(payload),
            "fixture '{name}' contains a live URL: {payload}"
        );
    }
}

#[test]
fn test_live_url_detection_works() {
    assert!(contains_live_url("https://api.openai.com/v1/chat"));
}

#[test]
fn test_localhost_url_not_flagged() {
    assert!(!contains_live_url("http://localhost:11434"));
}

#[test]
fn test_fixture_count_matches_expected() {
    assert_eq!(FIXTURE_PAYLOADS.len(), 10);
}

#[test]
fn test_all_fixture_names_unique() {
    let mut names: Vec<&str> = FIXTURE_PAYLOADS.iter().map(|(n, _)| *n).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), FIXTURE_PAYLOADS.len());
}

#[test]
fn test_all_payloads_are_valid_json_structure() {
    for (_, payload) in FIXTURE_PAYLOADS {
        assert!(payload.starts_with('{'));
        assert!(payload.ends_with('}'));
    }
}
