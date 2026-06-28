// Security: no live keys -- verify no real API key patterns appear in test fixtures.

const FIXTURE_BLOBS: &[&str] = &[
    r#"{"model":"claude-3-5-haiku","input_tokens":100}"#,
    r#"{"activity_key":"web_search","replayed":true}"#,
    r#"{"run_id":"test-001","status":"Completed"}"#,
    r#"{"trace_id":"0af7651916cd43dd8448eb211c80319c"}"#,
    r#"{"provider":"local","model":"qwen3"}"#,
];

fn looks_like_live_key(s: &str) -> bool {
    let prefixes = ["sk-ant-", "sk-", "AIza", "Bearer ya29", "ghp_", "xoxb-"];
    prefixes.iter().any(|p| s.contains(p))
}

fn has_high_entropy_64char_string(s: &str) -> bool {
    // crude check: 64-char hex or base64 substrings that aren't known-safe test values
    let known_safe = ["0af7651916cd43dd8448eb211c80319c"];
    let tokens: Vec<&str> = s.split(|c: char| !c.is_alphanumeric() && c != '-' && c != '_').collect();
    for tok in tokens {
        if tok.len() >= 40 && !known_safe.contains(&tok) {
            return true;
        }
    }
    false
}

#[test]
fn test_no_live_key_prefix_in_fixtures() {
    for blob in FIXTURE_BLOBS {
        assert!(!looks_like_live_key(blob), "live key found in: {blob}");
    }
}

#[test]
fn test_anthropic_key_prefix_detected() {
    assert!(looks_like_live_key("sk-ant-api03-abc123"));
}

#[test]
fn test_openai_key_prefix_detected() {
    assert!(looks_like_live_key("sk-proj-abc123"));
}

#[test]
fn test_no_high_entropy_strings_in_fixtures() {
    for blob in FIXTURE_BLOBS {
        assert!(!has_high_entropy_64char_string(blob), "suspicious string in: {blob}");
    }
}

#[test]
fn test_github_pat_prefix_detected() {
    assert!(looks_like_live_key("ghp_something123"));
}

#[test]
fn test_normal_uuid_not_flagged_as_live_key() {
    let s = r#"{"run_id":"550e8400-e29b-41d4-a716-446655440000"}"#;
    assert!(!looks_like_live_key(s));
}
