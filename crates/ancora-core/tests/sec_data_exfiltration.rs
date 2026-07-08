// Security: data exfiltration guard -- block PII in outbound tool calls.

struct PiiGuard {
    patterns: Vec<&'static str>,
}

impl PiiGuard {
    fn new() -> Self {
        Self {
            patterns: vec![
                "ssn:",
                "credit_card:",
                "password:",
                "api_key:",
                "secret:",
                "token:",
            ],
        }
    }

    fn scan(&self, payload: &str) -> Vec<String> {
        let lower = payload.to_lowercase().replace('"', "");
        self.patterns
            .iter()
            .filter(|p| lower.contains(*p))
            .map(|p| format!("pii-detected: {p}"))
            .collect()
    }

    fn is_safe(&self, payload: &str) -> bool {
        self.scan(payload).is_empty()
    }
}

#[test]
fn test_safe_payload_passes() {
    let g = PiiGuard::new();
    assert!(g.is_safe(r#"{"query": "tell me about Paris"}"#));
}

#[test]
fn test_ssn_in_payload_flagged() {
    let g = PiiGuard::new();
    assert!(!g.is_safe(r#"{"ssn": "123-45-6789"}"#));
}

#[test]
fn test_api_key_flagged() {
    let g = PiiGuard::new();
    let findings = g.scan(r#"{"api_key": "sk-abc123"}"#);
    assert!(!findings.is_empty());
    assert!(findings[0].contains("api_key"));
}

#[test]
fn test_multiple_pii_all_detected() {
    let g = PiiGuard::new();
    let findings = g.scan("password: hunter2 token: abc");
    assert_eq!(findings.len(), 2);
}

#[test]
fn test_case_insensitive_pii_detection() {
    let g = PiiGuard::new();
    assert!(!g.is_safe("SECRET: my-secret-value"));
}

#[test]
fn test_empty_payload_is_safe() {
    let g = PiiGuard::new();
    assert!(g.is_safe(""));
}

#[test]
fn test_normal_json_with_no_pii_safe() {
    let g = PiiGuard::new();
    let payload = r#"{"model": "claude-3", "max_tokens": 1024, "user": "alice"}"#;
    assert!(g.is_safe(payload));
}
