use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Anthropic provider profile.
///
/// Reads the API key from `ANTHROPIC_API_KEY` at call time.
/// The `anthropic-version` header is pinned to `2023-06-01` via `extra_headers`.
pub fn build_anthropic_profile() -> ProviderProfile {
    ProviderProfile::new(
        "anthropic",
        "https://api.anthropic.com",
        AuthStrategy::HeaderKey {
            header: "x-api-key".to_owned(),
            env_var: "ANTHROPIC_API_KEY".to_owned(),
        },
    )
    .with_chat_path("/v1/messages")
    .with_extra_header("anthropic-version", "2023-06-01")
    .add_model(
        ModelMeta::new("claude-opus-4-8", 200_000)
            .with_pricing(15.0, 75.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("claude-sonnet-4-6", 200_000)
            .with_pricing(3.0, 15.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("claude-haiku-4-5", 200_000)
            .with_pricing(0.80, 4.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_alias("claude-3-5-sonnet", "claude-sonnet-4-6")
    .add_alias("claude-3-5-haiku", "claude-haiku-4-5")
    .add_alias("claude-opus", "claude-opus-4-8")
}

#[cfg(test)]
const FIXTURE_CHAT: &str = r#"{"id":"msg_01","type":"message","role":"assistant","content":[{"type":"text","text":"Hello from Anthropic"}],"model":"claude-sonnet-4-6","stop_reason":"end_turn","usage":{"input_tokens":12,"output_tokens":4}}"#;

#[cfg(test)]
const FIXTURE_TOOL_CALL: &str = r#"{"id":"msg_02","type":"message","role":"assistant","content":[{"type":"tool_use","id":"toolu_01XYZ","name":"get_weather","input":{"city":"Paris"}}],"stop_reason":"tool_use","usage":{"input_tokens":25,"output_tokens":12}}"#;

#[cfg(test)]
const FIXTURE_STREAM_LINES: &[&str] = &[
    r#"data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}"#,
    r#"data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" world"}}"#,
    r#"data: {"type":"message_delta","delta":{"stop_reason":"end_turn"},"usage":{"output_tokens":5}}"#,
    r#"data: {"type":"message_stop"}"#,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::anthropic::AnthropicClient;
    use std::sync::Arc;

    fn client() -> AnthropicClient {
        AnthropicClient::new(Arc::new(build_anthropic_profile()))
    }

    #[test]
    fn anthropic_recorded_fixture_completes() {
        let resp = client().parse_response(FIXTURE_CHAT, "claude-sonnet-4-6").unwrap();
        assert_eq!(resp.content, "Hello from Anthropic");
        assert_eq!(resp.tokens_in, 12);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn anthropic_cost_computed_from_profile_pricing() {
        let resp = client().parse_response(FIXTURE_CHAT, "claude-sonnet-4-6").unwrap();
        // 12 * $3.0/M + 4 * $15.0/M
        let expected = 12.0 * 3.0 / 1_000_000.0 + 4.0 * 15.0 / 1_000_000.0;
        let cost = resp.cost_usd.expect("cost must be Some for priced model");
        assert!((cost - expected).abs() < 1e-12);
    }

    #[test]
    fn anthropic_profile_base_url_correct() {
        let p = build_anthropic_profile();
        assert_eq!(p.base_url, "https://api.anthropic.com");
    }

    #[test]
    fn anthropic_profile_version_header_present() {
        let p = build_anthropic_profile();
        assert_eq!(
            p.extra_headers.get("anthropic-version").map(|s| s.as_str()),
            Some("2023-06-01")
        );
    }

    #[test]
    fn anthropic_profile_has_opus_model() {
        let p = build_anthropic_profile();
        let meta = p.model_meta("claude-opus-4-8").unwrap();
        assert_eq!(meta.context_window, 200_000);
        assert!(meta.capabilities.tools);
        assert!(meta.capabilities.vision);
    }

    #[test]
    fn anthropic_alias_claude_opus_resolves() {
        let p = build_anthropic_profile();
        assert_eq!(p.resolve_model_id("claude-opus"), "claude-opus-4-8");
    }

    #[test]
    fn anthropic_tool_call_parsed_from_fixture() {
        let resp = client().parse_response(FIXTURE_TOOL_CALL, "claude-sonnet-4-6").unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        let tc = &resp.tool_calls[0];
        assert_eq!(tc.id, "toolu_01XYZ");
        assert_eq!(tc.function.name, "get_weather");
        assert!(tc.function.arguments.contains("Paris"));
    }

    #[test]
    fn anthropic_tool_call_content_is_empty() {
        let resp = client().parse_response(FIXTURE_TOOL_CALL, "claude-sonnet-4-6").unwrap();
        assert!(resp.content.is_empty());
    }

    #[test]
    fn anthropic_request_body_contains_tools_array() {
        use crate::types::{CompletionRequest, FunctionDefinition, Message, ToolDefinition};
        let req = CompletionRequest {
            model_id: "claude-sonnet-4-6".to_owned(),
            messages: vec![Message::text("user", "What is the weather?")],
            max_tokens: Some(1024),
            temperature: None,
            tools: vec![ToolDefinition {
                kind: "function".to_owned(),
                function: FunctionDefinition {
                    name: "get_weather".to_owned(),
                    description: "Get current weather".to_owned(),
                    parameters: serde_json::json!({"type": "object", "properties": {"city": {"type": "string"}}}),
                },
            }],
            tool_choice: None,
        };
        let body = client().build_request_body(&req, false).unwrap();
        let tools = body["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "get_weather");
        assert!(tools[0]["input_schema"].is_object());
    }

    #[test]
    fn anthropic_streaming_fixture_yields_ordered_tokens() {
        use crate::adapters::anthropic::parse_sse_line;
        let mut tokens = Vec::new();
        for line in FIXTURE_STREAM_LINES {
            if let Some(ev) = parse_sse_line(line) {
                tokens.push(ev);
            }
        }
        assert_eq!(tokens[0].text, "Hello");
        assert!(!tokens[0].finished);
        assert_eq!(tokens[1].text, " world");
        assert!(!tokens[1].finished);
        assert!(tokens[2].finished);
        assert!(tokens[2].text.is_empty());
    }

    #[test]
    fn anthropic_streaming_tokens_accumulate_in_order() {
        use crate::adapters::anthropic::parse_sse_line;
        let combined: String = FIXTURE_STREAM_LINES
            .iter()
            .filter_map(|l| parse_sse_line(l))
            .filter(|ev| !ev.finished)
            .map(|ev| ev.text)
            .collect();
        assert_eq!(combined, "Hello world");
    }

    #[test]
    fn anthropic_stream_emits_only_one_finished_event() {
        use crate::adapters::anthropic::parse_sse_line;
        let finished_count = FIXTURE_STREAM_LINES
            .iter()
            .filter_map(|l| parse_sse_line(l))
            .filter(|ev| ev.finished)
            .count();
        assert!(finished_count >= 1);
    }

    #[test]
    fn anthropic_request_body_system_at_top_level() {
        use crate::types::{CompletionRequest, Message};
        let req = CompletionRequest::simple(
            "claude-sonnet-4-6",
            vec![
                Message::text("system", "You are helpful"),
                Message::text("user", "Hello"),
            ],
        );
        let body = client().build_request_body(&req, false).unwrap();
        assert_eq!(body["system"], "You are helpful");
        let messages = body["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "user");
    }
}
