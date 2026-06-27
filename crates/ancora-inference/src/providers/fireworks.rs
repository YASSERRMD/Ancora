use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Fireworks AI provider profile.
///
/// Fireworks AI provides an OpenAI-compatible inference endpoint optimized
/// for open-source models. Use with `OpenAiClient`.
/// Reads the API key from `FIREWORKS_API_KEY` at call time.
pub fn build_fireworks_profile() -> ProviderProfile {
    ProviderProfile::new(
        "fireworks",
        "https://api.fireworks.ai/inference",
        AuthStrategy::BearerToken { env_var: "FIREWORKS_API_KEY".to_owned() },
    )
    // Llama 3.1 family
    .add_model(
        ModelMeta::new("accounts/fireworks/models/llama-v3p1-70b-instruct", 131_072)
            .with_pricing(0.90, 0.90)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("accounts/fireworks/models/llama-v3p1-8b-instruct", 131_072)
            .with_pricing(0.20, 0.20)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("accounts/fireworks/models/llama-v3p1-405b-instruct", 131_072)
            .with_pricing(3.00, 3.00)
            .with_tools()
            .with_streaming(),
    )
    // Mixtral
    .add_model(
        ModelMeta::new("accounts/fireworks/models/mixtral-8x22b-instruct", 65_536)
            .with_pricing(1.20, 1.20)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("accounts/fireworks/models/mixtral-8x7b-instruct", 32_768)
            .with_pricing(0.50, 0.50)
            .with_streaming(),
    )
    // Qwen
    .add_model(
        ModelMeta::new("accounts/fireworks/models/qwen2p5-72b-instruct", 32_768)
            .with_pricing(0.90, 0.90)
            .with_tools()
            .with_streaming(),
    )
    // Aliases
    .add_alias("llama3.1-70b", "accounts/fireworks/models/llama-v3p1-70b-instruct")
    .add_alias("llama3.1-8b", "accounts/fireworks/models/llama-v3p1-8b-instruct")
    .add_alias("llama3.1-405b", "accounts/fireworks/models/llama-v3p1-405b-instruct")
    .add_alias("mixtral-8x22b", "accounts/fireworks/models/mixtral-8x22b-instruct")
    .add_alias("mixtral", "accounts/fireworks/models/mixtral-8x7b-instruct")
    .add_alias("qwen2.5-72b", "accounts/fireworks/models/qwen2p5-72b-instruct")
}

#[cfg(test)]
const FIREWORKS_FIXTURE: &str = r#"{"id":"chatcmpl-fw-01","choices":[{"message":{"role":"assistant","content":"Hello from Fireworks","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":4}}"#;

#[cfg(test)]
const FIREWORKS_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" Fireworks"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fireworks_provider_name_is_fireworks() {
        assert_eq!(build_fireworks_profile().name, "fireworks");
    }

    #[test]
    fn fireworks_base_url_is_correct() {
        assert_eq!(
            build_fireworks_profile().base_url,
            "https://api.fireworks.ai/inference"
        );
    }

    #[test]
    fn fireworks_llama_70b_alias_resolves() {
        let p = build_fireworks_profile();
        assert_eq!(
            p.resolve_model_id("llama3.1-70b"),
            "accounts/fireworks/models/llama-v3p1-70b-instruct"
        );
    }

    #[test]
    fn fireworks_mixtral_alias_resolves() {
        let p = build_fireworks_profile();
        assert_eq!(
            p.resolve_model_id("mixtral"),
            "accounts/fireworks/models/mixtral-8x7b-instruct"
        );
    }

    #[test]
    fn fireworks_llama_70b_has_tools() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/llama-v3p1-70b-instruct").unwrap();
        assert!(m.capabilities.tools);
    }

    #[test]
    fn fireworks_llama_8b_has_no_tools() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/llama-v3p1-8b-instruct").unwrap();
        assert!(!m.capabilities.tools);
    }

    #[test]
    fn fireworks_all_models_stream() {
        let p = build_fireworks_profile();
        let ids = [
            "accounts/fireworks/models/llama-v3p1-70b-instruct",
            "accounts/fireworks/models/llama-v3p1-8b-instruct",
            "accounts/fireworks/models/llama-v3p1-405b-instruct",
            "accounts/fireworks/models/mixtral-8x22b-instruct",
            "accounts/fireworks/models/mixtral-8x7b-instruct",
            "accounts/fireworks/models/qwen2p5-72b-instruct",
        ];
        for id in ids {
            let m = p.model_meta(id).unwrap_or_else(|| panic!("{id} missing"));
            assert!(m.capabilities.streaming, "{id} must stream");
        }
    }

    #[test]
    fn fireworks_llama_large_context_window() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/llama-v3p1-405b-instruct").unwrap();
        assert_eq!(m.context_window, 131_072);
    }

    #[test]
    fn fireworks_qwen_has_tools() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/qwen2p5-72b-instruct").unwrap();
        assert!(m.capabilities.tools);
    }

    #[test]
    fn fireworks_405b_alias_resolves() {
        let p = build_fireworks_profile();
        assert_eq!(
            p.resolve_model_id("llama3.1-405b"),
            "accounts/fireworks/models/llama-v3p1-405b-instruct"
        );
    }

    #[test]
    fn fireworks_cost_summary_correct_for_llama_70b() {
        let resp = fw_client()
            .parse_response(
                FIREWORKS_FIXTURE,
                "accounts/fireworks/models/llama-v3p1-70b-instruct",
            )
            .unwrap();
        // 9 in * $0.90/M + 4 out * $0.90/M
        let expected = (9.0 + 4.0) * 0.90 / 1_000_000.0;
        let cost = resp.cost_usd.unwrap();
        assert!((cost - expected).abs() < 1e-12);
    }

    #[test]
    fn fireworks_llama_70b_has_pricing() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/llama-v3p1-70b-instruct").unwrap();
        assert!(m.pricing.is_some());
    }

    #[test]
    fn fireworks_llama_8b_cheaper_than_405b() {
        let p = build_fireworks_profile();
        let large = p.model_meta("accounts/fireworks/models/llama-v3p1-405b-instruct").unwrap();
        let small = p.model_meta("accounts/fireworks/models/llama-v3p1-8b-instruct").unwrap();
        let lp = large.pricing.as_ref().unwrap();
        let sp = small.pricing.as_ref().unwrap();
        assert!(sp.input_per_million < lp.input_per_million);
    }

    fn fw_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_fireworks_profile()))
    }

    #[test]
    fn fireworks_function_calling_request_body_has_tools() {
        use crate::types::{CompletionRequest, FunctionDefinition, Message, ToolDefinition};
        let mut req = CompletionRequest::simple(
            "llama3.1-70b",
            vec![Message::text("user", "What is the weather?")],
        );
        req.tools = vec![ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: "get_weather".to_owned(),
                description: "Get weather for location".to_owned(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {"location": {"type": "string"}},
                    "required": ["location"]
                }),
            },
        }];
        let body = fw_client().build_request_body(&req, false).unwrap();
        assert!(body["tools"].is_array());
        assert_eq!(body["tools"][0]["type"], "function");
        assert_eq!(body["tools"][0]["function"]["name"], "get_weather");
    }

    #[test]
    fn fireworks_recorded_fixture_completes() {
        let resp = fw_client()
            .parse_response(
                FIREWORKS_FIXTURE,
                "accounts/fireworks/models/llama-v3p1-70b-instruct",
            )
            .unwrap();
        assert_eq!(resp.content, "Hello from Fireworks");
        assert_eq!(resp.tokens_in, 9);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn fireworks_fixture_no_tool_calls() {
        let resp = fw_client()
            .parse_response(
                FIREWORKS_FIXTURE,
                "accounts/fireworks/models/llama-v3p1-70b-instruct",
            )
            .unwrap();
        assert!(resp.tool_calls.is_empty());
    }

    #[test]
    fn fireworks_function_calling_model_resolved() {
        use crate::types::{CompletionRequest, Message};
        let req = CompletionRequest::simple("llama3.1-70b", vec![Message::text("user", "Hi")]);
        let body = fw_client().build_request_body(&req, false).unwrap();
        assert_eq!(
            body["model"],
            "accounts/fireworks/models/llama-v3p1-70b-instruct"
        );
    }
}
