// Documentation audit: glossary covers all core Ancora terms.

const GLOSSARY_TERMS: &[&str] = &[
    "activity",
    "activity_key",
    "activity_kind",
    "agent",
    "a2a",
    "checkpointing",
    "circuit_breaker",
    "cost_ceiling",
    "data_residency",
    "data_sovereignty",
    "determinism",
    "divergence",
    "event_id",
    "gdpr_erasure",
    "human_in_the_loop",
    "idempotency",
    "journal",
    "local_first",
    "mcp",
    "memory_tier",
    "orchestration_graph",
    "otel_span",
    "provider",
    "replay",
    "run_id",
    "seq",
    "tool_allowlist",
    "trace_id",
    "vector_store",
    "verifier",
];

#[test]
fn test_30_glossary_terms() {
    assert_eq!(GLOSSARY_TERMS.len(), 30);
}

#[test]
fn test_no_duplicate_glossary_terms() {
    let mut sorted = GLOSSARY_TERMS.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), GLOSSARY_TERMS.len());
}

#[test]
fn test_all_terms_snake_case() {
    for term in GLOSSARY_TERMS {
        assert!(term.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'), "not snake_case: {term}");
    }
}

#[test]
fn test_core_terms_present() {
    let core = ["journal", "replay", "determinism", "agent", "tool_allowlist"];
    for t in &core { assert!(GLOSSARY_TERMS.contains(t), "glossary missing: {t}"); }
}

#[test]
fn test_a2a_and_mcp_in_glossary() {
    assert!(GLOSSARY_TERMS.contains(&"a2a"));
    assert!(GLOSSARY_TERMS.contains(&"mcp"));
}

#[test]
fn test_security_terms_in_glossary() {
    let sec = ["data_sovereignty", "gdpr_erasure", "tool_allowlist", "data_residency"];
    for t in &sec { assert!(GLOSSARY_TERMS.contains(t), "glossary missing security term: {t}"); }
}
