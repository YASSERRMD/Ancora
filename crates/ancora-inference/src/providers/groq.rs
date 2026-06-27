use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Groq provider profile.
///
/// Groq exposes an OpenAI-compatible endpoint backed by LPU hardware for
/// very low latency. Use with `OpenAiClient`. Reads `GROQ_API_KEY`.
///
/// Groq's base URL already contains the `/openai` prefix so the standard
/// `/v1/chat/completions` path appends correctly.
pub fn build_groq_profile() -> ProviderProfile {
    ProviderProfile::new(
        "groq",
        "https://api.groq.com/openai",
        AuthStrategy::BearerToken { env_var: "GROQ_API_KEY".to_owned() },
    )
    // Llama 3 family
    .add_model(
        ModelMeta::new("llama-3.3-70b-versatile", 128_000)
            .with_pricing(0.59, 0.79)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("llama-3.1-8b-instant", 128_000)
            .with_pricing(0.05, 0.08)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("llama3-70b-8192", 8_192)
            .with_pricing(0.59, 0.79)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("llama3-8b-8192", 8_192)
            .with_pricing(0.05, 0.08)
            .with_streaming(),
    )
    // Mixtral
    .add_model(
        ModelMeta::new("mixtral-8x7b-32768", 32_768)
            .with_pricing(0.24, 0.24)
            .with_streaming(),
    )
    // Gemma
    .add_model(
        ModelMeta::new("gemma2-9b-it", 8_192)
            .with_pricing(0.20, 0.20)
            .with_streaming(),
    )
    // Aliases
    .add_alias("llama-3.3-70b", "llama-3.3-70b-versatile")
    .add_alias("llama-3.1-8b", "llama-3.1-8b-instant")
    .add_alias("llama3-70b", "llama3-70b-8192")
    .add_alias("llama3-8b", "llama3-8b-8192")
    .add_alias("mixtral", "mixtral-8x7b-32768")
    .add_alias("gemma2", "gemma2-9b-it")
}

#[cfg(test)]
const GROQ_FIXTURE: &str = r#"{"id":"chatcmpl-groq-01","choices":[{"message":{"role":"assistant","content":"Hello from Groq","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":5}}"#;

/// Groq uses the standard OpenAI SSE format.
/// `OpenAiClient::parse_sse_line` handles the stream without modification.
/// This constant documents the expected line format for offline tests.
#[cfg(test)]
const GROQ_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" Groq"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groq_provider_name_is_groq() {
        assert_eq!(build_groq_profile().name, "groq");
    }

    #[test]
    fn groq_base_url_is_correct() {
        assert_eq!(build_groq_profile().base_url, "https://api.groq.com/openai");
    }

    #[test]
    fn groq_llama3_70b_alias_resolves() {
        let p = build_groq_profile();
        assert_eq!(p.resolve_model_id("llama3-70b"), "llama3-70b-8192");
    }

    #[test]
    fn groq_llama3_versatile_alias_resolves() {
        let p = build_groq_profile();
        assert_eq!(p.resolve_model_id("llama-3.3-70b"), "llama-3.3-70b-versatile");
    }

    #[test]
    fn groq_mixtral_alias_resolves() {
        let p = build_groq_profile();
        assert_eq!(p.resolve_model_id("mixtral"), "mixtral-8x7b-32768");
    }

    #[test]
    fn groq_llama3_70b_has_tools() {
        let p = build_groq_profile();
        let meta = p.model_meta("llama3-70b-8192").unwrap();
        assert!(meta.capabilities.tools);
    }

    #[test]
    fn groq_llama3_8b_has_no_tools() {
        let p = build_groq_profile();
        let meta = p.model_meta("llama3-8b-8192").unwrap();
        assert!(!meta.capabilities.tools);
    }

    #[test]
    fn groq_all_models_have_streaming() {
        let p = build_groq_profile();
        let ids = [
            "llama-3.3-70b-versatile",
            "llama-3.1-8b-instant",
            "llama3-70b-8192",
            "llama3-8b-8192",
            "mixtral-8x7b-32768",
            "gemma2-9b-it",
        ];
        for id in ids {
            let m = p.model_meta(id).unwrap_or_else(|| panic!("{id} missing"));
            assert!(m.capabilities.streaming, "{id} should have streaming");
        }
    }

    #[test]
    fn groq_llama3_70b_context_window() {
        let p = build_groq_profile();
        let meta = p.model_meta("llama3-70b-8192").unwrap();
        assert_eq!(meta.context_window, 8_192);
    }

    #[test]
    fn groq_llama_versatile_large_context() {
        let p = build_groq_profile();
        let meta = p.model_meta("llama-3.3-70b-versatile").unwrap();
        assert_eq!(meta.context_window, 128_000);
    }

    #[test]
    fn groq_streaming_uses_openai_sse_format() {
        use crate::openai::OpenAiClient;
        let texts: Vec<String> = GROQ_STREAM_LINES.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(texts, vec!["Hello", " Groq"]);
    }

    #[test]
    fn groq_cost_summary_correct_for_llama3_70b() {
        let resp = groq_client().parse_response(GROQ_FIXTURE, "llama3-70b-8192").unwrap();
        // 10 in * $0.59/M + 5 out * $0.79/M
        let expected = 10.0 * 0.59 / 1_000_000.0 + 5.0 * 0.79 / 1_000_000.0;
        let cost = resp.cost_usd.unwrap();
        assert!((cost - expected).abs() < 1e-12);
    }

    #[test]
    fn groq_llama3_70b_has_pricing() {
        let p = build_groq_profile();
        let m = p.model_meta("llama3-70b-8192").unwrap();
        assert!(m.pricing.is_some());
    }

    #[test]
    fn groq_small_model_cheaper_than_large() {
        let p = build_groq_profile();
        let large = p.model_meta("llama-3.3-70b-versatile").unwrap();
        let small = p.model_meta("llama-3.1-8b-instant").unwrap();
        let lp = large.pricing.as_ref().unwrap();
        let sp = small.pricing.as_ref().unwrap();
        assert!(sp.input_per_million < lp.input_per_million);
    }

    #[test]
    fn groq_stream_done_emits_finished() {
        use crate::openai::OpenAiClient;
        let ev = OpenAiClient::parse_sse_line("data: [DONE]").unwrap();
        assert!(ev.finished);
    }

    fn groq_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_groq_profile()))
    }

    #[test]
    fn groq_recorded_fixture_completes() {
        let resp = groq_client().parse_response(GROQ_FIXTURE, "llama3-70b-8192").unwrap();
        assert_eq!(resp.content, "Hello from Groq");
        assert_eq!(resp.tokens_in, 10);
        assert_eq!(resp.tokens_out, 5);
    }

    #[test]
    fn groq_fixture_no_tool_calls() {
        let resp = groq_client().parse_response(GROQ_FIXTURE, "llama3-70b-8192").unwrap();
        assert!(resp.tool_calls.is_empty());
    }

    #[test]
    fn groq_fixture_content_non_empty() {
        let resp = groq_client().parse_response(GROQ_FIXTURE, "llama3-70b-8192").unwrap();
        assert!(!resp.content.is_empty());
    }
}
