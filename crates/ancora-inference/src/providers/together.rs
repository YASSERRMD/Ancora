use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Together AI provider profile.
///
/// Together AI hosts open-source models on OpenAI-compatible endpoints.
/// Use with `OpenAiClient`. Reads the API key from `TOGETHER_API_KEY`.
pub fn build_together_profile() -> ProviderProfile {
    ProviderProfile::new(
        "together",
        "https://api.together.xyz",
        AuthStrategy::BearerToken { env_var: "TOGETHER_API_KEY".to_owned() },
    )
    // Llama 3 family
    .add_model(
        ModelMeta::new("meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo", 128_000)
            .with_pricing(0.88, 0.88)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo", 128_000)
            .with_pricing(0.18, 0.18)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("meta-llama/Llama-3-70b-chat-hf", 8_192)
            .with_pricing(0.90, 0.90)
            .with_streaming(),
    )
    // Mistral family
    .add_model(
        ModelMeta::new("mistralai/Mixtral-8x7B-Instruct-v0.1", 32_768)
            .with_pricing(0.60, 0.60)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("mistralai/Mistral-7B-Instruct-v0.3", 32_768)
            .with_pricing(0.20, 0.20)
            .with_streaming(),
    )
    // Qwen
    .add_model(
        ModelMeta::new("Qwen/Qwen2.5-72B-Instruct-Turbo", 32_768)
            .with_pricing(1.20, 1.20)
            .with_tools()
            .with_streaming(),
    )
    // DeepSeek (hosted by Together)
    .add_model(
        ModelMeta::new("deepseek-ai/DeepSeek-R1", 64_000)
            .with_pricing(3.00, 7.00)
            .with_streaming(),
    )
    // Aliases
    .add_alias("llama3.1-70b", "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo")
    .add_alias("llama3.1-8b", "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo")
    .add_alias("llama3-70b", "meta-llama/Llama-3-70b-chat-hf")
    .add_alias("mixtral", "mistralai/Mixtral-8x7B-Instruct-v0.1")
    .add_alias("mistral-7b", "mistralai/Mistral-7B-Instruct-v0.3")
    .add_alias("qwen2.5-72b", "Qwen/Qwen2.5-72B-Instruct-Turbo")
    .add_alias("deepseek-r1", "deepseek-ai/DeepSeek-R1")
}

#[cfg(test)]
const TOGETHER_FIXTURE: &str = r#"{"id":"chatcmpl-together-01","choices":[{"message":{"role":"assistant","content":"Hello from Together","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":8,"completion_tokens":4}}"#;

#[cfg(test)]
const TOGETHER_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" Together"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn together_provider_name_is_together() {
        assert_eq!(build_together_profile().name, "together");
    }

    #[test]
    fn together_base_url_is_correct() {
        assert_eq!(build_together_profile().base_url, "https://api.together.xyz");
    }

    #[test]
    fn together_llama3_70b_alias_resolves() {
        let p = build_together_profile();
        assert_eq!(
            p.resolve_model_id("llama3.1-70b"),
            "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo"
        );
    }

    #[test]
    fn together_mixtral_alias_resolves() {
        let p = build_together_profile();
        assert_eq!(
            p.resolve_model_id("mixtral"),
            "mistralai/Mixtral-8x7B-Instruct-v0.1"
        );
    }

    #[test]
    fn together_deepseek_alias_resolves() {
        let p = build_together_profile();
        assert_eq!(p.resolve_model_id("deepseek-r1"), "deepseek-ai/DeepSeek-R1");
    }

    #[test]
    fn together_llama3_70b_has_tools() {
        let p = build_together_profile();
        let meta = p.model_meta("meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo").unwrap();
        assert!(meta.capabilities.tools);
    }

    #[test]
    fn together_mixtral_has_no_tools() {
        let p = build_together_profile();
        let meta = p.model_meta("mistralai/Mixtral-8x7B-Instruct-v0.1").unwrap();
        assert!(!meta.capabilities.tools);
    }

    #[test]
    fn together_all_models_stream() {
        let p = build_together_profile();
        let ids = [
            "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo",
            "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo",
            "meta-llama/Llama-3-70b-chat-hf",
            "mistralai/Mixtral-8x7B-Instruct-v0.1",
            "mistralai/Mistral-7B-Instruct-v0.3",
            "Qwen/Qwen2.5-72B-Instruct-Turbo",
            "deepseek-ai/DeepSeek-R1",
        ];
        for id in ids {
            let m = p.model_meta(id).unwrap_or_else(|| panic!("{id} missing"));
            assert!(m.capabilities.streaming, "{id} should stream");
        }
    }

    #[test]
    fn together_llama_has_pricing() {
        let p = build_together_profile();
        let m = p.model_meta("meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo").unwrap();
        assert!(m.pricing.is_some());
    }

    #[test]
    fn together_llama_8b_cheaper_than_70b() {
        let p = build_together_profile();
        let large = p.model_meta("meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo").unwrap();
        let small = p.model_meta("meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo").unwrap();
        let lp = large.pricing.as_ref().unwrap();
        let sp = small.pricing.as_ref().unwrap();
        assert!(sp.input_per_million < lp.input_per_million);
    }

    #[test]
    fn together_llama31_large_context() {
        let p = build_together_profile();
        let m = p.model_meta("meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo").unwrap();
        assert_eq!(m.context_window, 128_000);
    }

    #[test]
    fn together_request_body_includes_tools_array() {
        use crate::openai::OpenAiClient;
        use crate::types::{CompletionRequest, FunctionDefinition, Message, ToolDefinition};
        use std::sync::Arc;
        let client = OpenAiClient::new(Arc::new(build_together_profile()));
        let mut req = CompletionRequest::simple(
            "llama3.1-70b",
            vec![Message::text("user", "What is 2+2?")],
        );
        req.tools = vec![ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: "calculator".to_owned(),
                description: "Performs arithmetic".to_owned(),
                parameters: serde_json::json!({"type":"object","properties":{}}),
            },
        }];
        let body = client.build_request_body(&req, false).unwrap();
        assert!(body["tools"].is_array());
        assert_eq!(body["tools"][0]["function"]["name"], "calculator");
    }

    fn together_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_together_profile()))
    }

    #[test]
    fn together_recorded_fixture_completes() {
        let resp = together_client()
            .parse_response(TOGETHER_FIXTURE, "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo")
            .unwrap();
        assert_eq!(resp.content, "Hello from Together");
        assert_eq!(resp.tokens_in, 8);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn together_fixture_no_tool_calls() {
        let resp = together_client()
            .parse_response(TOGETHER_FIXTURE, "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo")
            .unwrap();
        assert!(resp.tool_calls.is_empty());
    }

    #[test]
    fn together_request_body_model_resolved() {
        use crate::openai::OpenAiClient;
        use crate::types::{CompletionRequest, Message};
        use std::sync::Arc;
        let client = OpenAiClient::new(Arc::new(build_together_profile()));
        let req = CompletionRequest::simple(
            "llama3.1-70b",
            vec![Message::text("user", "Hi")],
        );
        let body = client.build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo");
    }
}
