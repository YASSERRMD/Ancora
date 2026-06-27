use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Google Gemini provider profile.
///
/// Reads the API key from `GOOGLE_API_KEY` at call time and passes it as
/// the `key` query parameter on every request. The URL template is
/// `https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={key}`.
/// The `GeminiClient` adapter constructs the model-specific path at call time.
pub fn build_gemini_profile() -> ProviderProfile {
    ProviderProfile::new(
        "gemini",
        "https://generativelanguage.googleapis.com",
        AuthStrategy::QueryParam {
            param: "key".to_owned(),
            env_var: "GOOGLE_API_KEY".to_owned(),
        },
    )
    .add_model(
        ModelMeta::new("gemini-2.0-flash", 1_000_000)
            .with_pricing(0.10, 0.40)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("gemini-2.5-pro", 2_000_000)
            .with_pricing(1.25, 10.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("gemini-1.5-flash", 1_000_000)
            .with_pricing(0.075, 0.30)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_alias("gemini-flash", "gemini-2.0-flash")
    .add_alias("gemini-pro", "gemini-2.5-pro")
}

#[cfg(test)]
const FIXTURE_CHAT: &str = r#"{"candidates":[{"content":{"role":"model","parts":[{"text":"Hello from Gemini"}]},"finishReason":"STOP"}],"usageMetadata":{"promptTokenCount":8,"candidatesTokenCount":4}}"#;

#[cfg(test)]
const FIXTURE_FUNCTION_CALL: &str = r#"{"candidates":[{"content":{"role":"model","parts":[{"functionCall":{"name":"get_weather","args":{"city":"Tokyo"}}}]},"finishReason":"STOP"}],"usageMetadata":{"promptTokenCount":20,"candidatesTokenCount":10}}"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::gemini::GeminiClient;
    use std::sync::Arc;

    fn client() -> GeminiClient {
        GeminiClient::new(Arc::new(build_gemini_profile()))
    }

    #[test]
    fn gemini_recorded_fixture_completes() {
        let resp = client().parse_response(FIXTURE_CHAT, "gemini-2.0-flash").unwrap();
        assert_eq!(resp.content, "Hello from Gemini");
        assert_eq!(resp.tokens_in, 8);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn gemini_cost_computed_from_profile_pricing() {
        let resp = client().parse_response(FIXTURE_CHAT, "gemini-2.0-flash").unwrap();
        // 8 * $0.10/M + 4 * $0.40/M
        let expected = 8.0 * 0.10 / 1_000_000.0 + 4.0 * 0.40 / 1_000_000.0;
        let cost = resp.cost_usd.expect("cost must be Some for priced model");
        assert!((cost - expected).abs() < 1e-12);
    }

    #[test]
    fn gemini_profile_base_url_correct() {
        let p = build_gemini_profile();
        assert_eq!(p.base_url, "https://generativelanguage.googleapis.com");
    }

    #[test]
    fn gemini_profile_uses_query_param_auth() {
        use crate::provider::AuthStrategy;
        let p = build_gemini_profile();
        assert!(matches!(p.auth, AuthStrategy::QueryParam { .. }));
    }

    #[test]
    fn gemini_alias_gemini_flash_resolves() {
        let p = build_gemini_profile();
        assert_eq!(p.resolve_model_id("gemini-flash"), "gemini-2.0-flash");
    }

    #[test]
    fn gemini_function_call_round_trip_works() {
        let resp = client().parse_response(FIXTURE_FUNCTION_CALL, "gemini-2.0-flash").unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        let tc = &resp.tool_calls[0];
        assert_eq!(tc.function.name, "get_weather");
        assert!(tc.function.arguments.contains("Tokyo"));
    }

    #[test]
    fn gemini_function_call_content_is_empty() {
        let resp = client().parse_response(FIXTURE_FUNCTION_CALL, "gemini-2.0-flash").unwrap();
        assert!(resp.content.is_empty());
    }

    #[test]
    fn gemini_request_body_wraps_tools_in_function_declarations() {
        use crate::types::{CompletionRequest, FunctionDefinition, Message, ToolDefinition};
        let req = CompletionRequest {
            model_id: "gemini-2.0-flash".to_owned(),
            messages: vec![Message::text("user", "What is the weather?")],
            max_tokens: None,
            temperature: None,
            tools: vec![ToolDefinition {
                kind: "function".to_owned(),
                function: FunctionDefinition {
                    name: "get_weather".to_owned(),
                    description: "Get current weather".to_owned(),
                    parameters: serde_json::json!({"type": "object"}),
                },
            }],
            tool_choice: None,
        };
        let body = client().build_request_body(&req).unwrap();
        let fds = body["tools"][0]["functionDeclarations"].as_array().unwrap();
        assert_eq!(fds.len(), 1);
        assert_eq!(fds[0]["name"], "get_weather");
    }

    #[test]
    fn gemini_flash_model_has_large_context() {
        let p = build_gemini_profile();
        let meta = p.model_meta("gemini-2.0-flash").unwrap();
        assert_eq!(meta.context_window, 1_000_000);
        assert!(meta.capabilities.vision);
    }
}
