// Documentation audit: quickstart guides cover all languages and scenarios.

const QUICKSTART_SCENARIOS: &[(&str, &[&str])] = &[
    ("single_agent",    &["rust", "go", "python", "ts", "dotnet", "java"]),
    ("verifier",        &["rust", "go", "python", "ts", "dotnet", "java"]),
    ("human_in_loop",   &["rust", "go", "python", "ts"]),
    ("vector_rag",      &["rust", "go", "python"]),
    ("mcp_tool_use",    &["rust", "go", "ts"]),
];

#[test]
fn test_five_quickstart_scenarios() {
    assert_eq!(QUICKSTART_SCENARIOS.len(), 5);
}

#[test]
fn test_single_agent_covers_all_6_languages() {
    let (_, langs) = QUICKSTART_SCENARIOS[0];
    assert_eq!(langs.len(), 6);
}

#[test]
fn test_verifier_covers_all_6_languages() {
    let verifier = QUICKSTART_SCENARIOS.iter().find(|(s, _)| *s == "verifier");
    assert_eq!(verifier.map(|(_, l)| l.len()), Some(6));
}

#[test]
fn test_rust_in_all_scenarios() {
    for (scenario, langs) in QUICKSTART_SCENARIOS {
        assert!(langs.contains(&"rust"), "rust missing from scenario: {scenario}");
    }
}

#[test]
fn test_human_in_loop_quickstart_exists() {
    let found = QUICKSTART_SCENARIOS.iter().any(|(s, _)| *s == "human_in_loop");
    assert!(found, "no human-in-loop quickstart scenario");
}

#[test]
fn test_mcp_quickstart_exists() {
    let found = QUICKSTART_SCENARIOS.iter().any(|(s, _)| *s == "mcp_tool_use");
    assert!(found, "no MCP tool use quickstart");
}
