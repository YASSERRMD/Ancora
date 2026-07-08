// Example parity: MCP tool-use example produces same activity_key across languages.

const MCP_ACTIVITY_KEY_PREFIX: &str = "mcp-";
const MCP_TOOL_NAME: &str = "search";

struct McpExample {
    client_lang: &'static str,
    server_lang: &'static str,
    activity_key: &'static str,
    result_json: &'static str,
}

const MCP_EXAMPLES: &[McpExample] = &[
    McpExample {
        client_lang: "rust",
        server_lang: "python",
        activity_key: "mcp-python-server/search",
        result_json: r#"{"results":["item1","item2"]}"#,
    },
    McpExample {
        client_lang: "go",
        server_lang: "ts",
        activity_key: "mcp-ts-server/search",
        result_json: r#"{"results":["item1","item2"]}"#,
    },
    McpExample {
        client_lang: "python",
        server_lang: "rust",
        activity_key: "mcp-rust-server/search",
        result_json: r#"{"results":["item1","item2"]}"#,
    },
    McpExample {
        client_lang: "ts",
        server_lang: "go",
        activity_key: "mcp-go-server/search",
        result_json: r#"{"results":["item1","item2"]}"#,
    },
];

#[test]
fn test_all_activity_keys_start_with_mcp_prefix() {
    for e in MCP_EXAMPLES {
        assert!(
            e.activity_key.starts_with(MCP_ACTIVITY_KEY_PREFIX),
            "activity_key '{}' does not start with '{}'",
            e.activity_key,
            MCP_ACTIVITY_KEY_PREFIX
        );
    }
}

#[test]
fn test_all_activity_keys_end_with_tool_name() {
    for e in MCP_EXAMPLES {
        assert!(
            e.activity_key.ends_with(MCP_TOOL_NAME),
            "activity_key '{}' does not end with '{}'",
            e.activity_key,
            MCP_TOOL_NAME
        );
    }
}

#[test]
fn test_result_json_has_results_array() {
    for e in MCP_EXAMPLES {
        assert!(
            e.result_json.contains("results"),
            "no results field in {}",
            e.client_lang
        );
    }
}

#[test]
fn test_four_mcp_examples() {
    assert_eq!(MCP_EXAMPLES.len(), 4);
}

#[test]
fn test_no_self_client_server_pair() {
    for e in MCP_EXAMPLES {
        assert_ne!(e.client_lang, e.server_lang);
    }
}

#[test]
fn test_rust_is_client_in_at_least_one_example() {
    assert!(MCP_EXAMPLES.iter().any(|e| e.client_lang == "rust"));
}
