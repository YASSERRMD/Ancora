use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Mistral AI provider profile.
///
/// Mistral's API is OpenAI-compatible; use it with `OpenAiClient`.
/// Reads the API key from `MISTRAL_API_KEY` at call time.
pub fn build_mistral_profile() -> ProviderProfile {
    ProviderProfile::new(
        "mistral",
        "https://api.mistral.ai",
        AuthStrategy::BearerToken { env_var: "MISTRAL_API_KEY".to_owned() },
    )
    .add_model(
        ModelMeta::new("mistral-large-latest", 128_000)
            .with_pricing(2.0, 6.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("mistral-small-latest", 32_000)
            .with_pricing(0.20, 0.60)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("open-mistral-7b", 32_000)
            .with_pricing(0.25, 0.25)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("codestral-latest", 32_000)
            .with_pricing(0.20, 0.60)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("mistral-large", "mistral-large-latest")
    .add_alias("mistral-small", "mistral-small-latest")
    .add_alias("codestral", "codestral-latest")
}

#[cfg(test)]
const FIXTURE: &str = r#"{"id":"chatcmpl-mistral-01","choices":[{"message":{"role":"assistant","content":"Hello from Mistral","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":8,"completion_tokens":4}}"#;

// Mistral uses the same OpenAI SSE format: `data: {...}` with choices[].delta.content
#[cfg(test)]
const FIXTURE_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" Mistral"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::OpenAiClient;
    use crate::types::{CompletionRequest, Message};
    use std::sync::Arc;

    fn client() -> OpenAiClient {
        OpenAiClient::new(Arc::new(build_mistral_profile()))
    }

    #[test]
    fn mistral_request_body_has_model_and_messages() {
        let req = CompletionRequest::simple(
            "mistral-large-latest",
            vec![Message::text("user", "Hello")],
        );
        let body = client().build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], "mistral-large-latest");
        assert!(body["messages"].is_array());
    }

    #[test]
    fn mistral_alias_resolved_in_request_body() {
        let req = CompletionRequest::simple("mistral-large", vec![Message::text("user", "Hi")]);
        let body = client().build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], "mistral-large-latest");
    }

    #[test]
    fn mistral_base_url_correct() {
        let p = build_mistral_profile();
        assert_eq!(p.base_url, "https://api.mistral.ai");
    }

    #[test]
    fn mistral_completions_path_is_openai_compatible() {
        let p = build_mistral_profile();
        assert!(p.completions_url(None).ends_with("/v1/chat/completions"));
    }

    #[test]
    fn mistral_streaming_sse_uses_openai_format() {
        // Mistral uses the identical SSE format as OpenAI; parse_sse_line handles both.
        let mut tokens = Vec::new();
        for line in FIXTURE_STREAM_LINES {
            if let Some(ev) = OpenAiClient::parse_sse_line(line) {
                tokens.push(ev);
            }
        }
        assert_eq!(tokens[0].text, "Hello");
        assert!(!tokens[0].finished);
        assert_eq!(tokens[1].text, " Mistral");
        assert!(tokens[1].finished);
    }

    #[test]
    fn mistral_stream_done_sentinel_emits_finished() {
        let ev = OpenAiClient::parse_sse_line("data: [DONE]").unwrap();
        assert!(ev.finished);
        assert!(ev.text.is_empty());
    }
}
