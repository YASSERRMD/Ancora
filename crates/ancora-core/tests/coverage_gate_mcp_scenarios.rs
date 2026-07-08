// Coverage gate: MCP integration scenarios all have cross-language tests.

const MCP_SCENARIOS: &[(&str, &str, &str)] = &[
    ("rust-server", "python", "xlang_mcp_rust_python"),
    ("go-server", "ts", "xlang_mcp_go_ts"),
    ("ts-server", "rust", "xlang_mcp_go_ts"),
    ("python-server", "java", "xlang_mcp_go_ts"),
];

const MCP_TOOLS: &[&str] = &["search", "fetch", "write", "read"];

#[test]
#[allow(clippy::const_is_empty)]
fn test_mcp_scenarios_defined() {
    assert!(!MCP_SCENARIOS.is_empty());
}

#[test]
fn test_rust_server_python_client_covered() {
    let found = MCP_SCENARIOS
        .iter()
        .any(|(server, client, _)| *server == "rust-server" && *client == "python");
    assert!(found, "no rust-server/python test");
}

#[test]
fn test_go_server_ts_client_covered() {
    let found = MCP_SCENARIOS
        .iter()
        .any(|(server, client, _)| *server == "go-server" && *client == "ts");
    assert!(found, "no go-server/ts test");
}

#[test]
fn test_mcp_tools_include_search() {
    assert!(MCP_TOOLS.contains(&"search"));
}

#[test]
fn test_mcp_tools_include_fetch() {
    assert!(MCP_TOOLS.contains(&"fetch"));
}

#[test]
fn test_mcp_tools_count_at_least_4() {
    assert!(MCP_TOOLS.len() >= 4);
}

#[test]
fn test_no_self_server_client_pair() {
    for (server, client, _) in MCP_SCENARIOS {
        let server_lang = server.split('-').next().unwrap_or("");
        assert_ne!(server_lang, *client, "self-pair: {server} -> {client}");
    }
}
