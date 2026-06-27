use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Fallback model identifiers for OpenRouter routing.
///
/// OpenRouter uses `provider/model` namespacing. A fallback list lets OpenRouter
/// try alternatives when the first choice is unavailable or rate-limited.
pub struct OpenRouterConfig {
    /// Primary model (e.g. `"openai/gpt-4o"`).
    pub model_id: String,
    /// Ordered fallback list tried when the primary is unavailable.
    pub fallback_models: Vec<String>,
    /// Application name sent in `X-Title` for attribution.
    pub app_name: String,
    /// Site URL sent in `HTTP-Referer`.
    pub site_url: String,
}

impl Default for OpenRouterConfig {
    fn default() -> Self {
        Self {
            model_id: "openai/gpt-4o".to_owned(),
            fallback_models: vec!["anthropic/claude-3-5-haiku".to_owned()],
            app_name: "Ancora".to_owned(),
            site_url: "https://github.com/YASSERRMD/Ancora".to_owned(),
        }
    }
}

/// Build an OpenRouter provider profile with app-attribution headers.
///
/// OpenRouter routes to any supported provider. Model IDs use the
/// `provider/model` format (e.g. `"openai/gpt-4o"`, `"anthropic/claude-3-5-haiku"`).
///
/// Reads the API key from `OPENROUTER_API_KEY` at call time.
pub fn build_openrouter_profile(config: OpenRouterConfig) -> ProviderProfile {
    let mut profile = ProviderProfile::new(
        "openrouter",
        "https://openrouter.ai/api",
        AuthStrategy::BearerToken { env_var: "OPENROUTER_API_KEY".to_owned() },
    )
    .with_extra_header("HTTP-Referer", config.site_url)
    .with_extra_header("X-Title", config.app_name);

    // Register the primary model and each fallback in the catalog.
    let all_models = std::iter::once(config.model_id.as_str())
        .chain(config.fallback_models.iter().map(|s| s.as_str()));
    for m in all_models {
        profile = profile.add_model(
            ModelMeta::new(m, 200_000).with_tools().with_vision().with_streaming(),
        );
    }

    // Add the fallback list via a request transform.
    if !config.fallback_models.is_empty() {
        let fallbacks = config.fallback_models.clone();
        profile = profile.with_request_transform(move |body| {
            body["models"] = serde_json::json!(fallbacks);
        });
    }

    profile
}

#[cfg(test)]
const FIXTURE: &str = r#"{"id":"gen-openrouter","choices":[{"message":{"role":"assistant","content":"Hello from OpenRouter","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":7,"completion_tokens":4}}"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::OpenAiClient;
    use std::sync::Arc;

    fn client() -> OpenAiClient {
        OpenAiClient::new(Arc::new(build_openrouter_profile(OpenRouterConfig::default())))
    }

    #[test]
    fn openrouter_recorded_fixture_completes() {
        let resp = client().parse_response(FIXTURE, "openai/gpt-4o").unwrap();
        assert_eq!(resp.content, "Hello from OpenRouter");
        assert_eq!(resp.tokens_in, 7);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn openrouter_routes_to_target_model_id() {
        use crate::types::{CompletionRequest, Message};
        let p = Arc::new(build_openrouter_profile(OpenRouterConfig {
            model_id: "anthropic/claude-3-5-haiku".to_owned(),
            fallback_models: vec![],
            app_name: "test".to_owned(),
            site_url: "https://test.example".to_owned(),
        }));
        let c = OpenAiClient::new(p);
        let req = CompletionRequest::simple(
            "anthropic/claude-3-5-haiku",
            vec![Message::text("user", "hi")],
        );
        let body = c.build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], serde_json::json!("anthropic/claude-3-5-haiku"));
    }

    #[test]
    fn openrouter_app_attribution_headers_present() {
        let p = build_openrouter_profile(OpenRouterConfig {
            model_id: "openai/gpt-4o".to_owned(),
            fallback_models: vec![],
            app_name: "MyApp".to_owned(),
            site_url: "https://myapp.test".to_owned(),
        });
        assert_eq!(p.extra_headers.get("HTTP-Referer").map(|s| s.as_str()), Some("https://myapp.test"));
        assert_eq!(p.extra_headers.get("X-Title").map(|s| s.as_str()), Some("MyApp"));
    }

    #[test]
    fn openrouter_fallback_list_in_request_body() {
        use crate::types::{CompletionRequest, Message};
        let p = Arc::new(build_openrouter_profile(OpenRouterConfig {
            model_id: "openai/gpt-4o".to_owned(),
            fallback_models: vec!["anthropic/claude-3-5-haiku".to_owned()],
            app_name: "t".to_owned(),
            site_url: "https://t.test".to_owned(),
        }));
        let c = OpenAiClient::new(p);
        let req = CompletionRequest::simple("openai/gpt-4o", vec![Message::text("user", "hi")]);
        let body = c.build_request_body(&req, false).unwrap();
        let models = body["models"].as_array().unwrap();
        assert!(models.contains(&serde_json::json!("anthropic/claude-3-5-haiku")));
    }

    #[test]
    fn openrouter_base_url_correct() {
        let p = build_openrouter_profile(OpenRouterConfig::default());
        assert_eq!(p.base_url, "https://openrouter.ai/api");
    }

    #[test]
    fn openrouter_model_id_passthrough_different_providers() {
        use crate::types::{CompletionRequest, Message};
        let models = [
            "openai/gpt-4o",
            "anthropic/claude-3-5-haiku",
            "mistralai/mistral-7b-instruct",
        ];
        for model_id in models {
            let p = Arc::new(build_openrouter_profile(OpenRouterConfig {
                model_id: model_id.to_owned(),
                fallback_models: vec![],
                app_name: "t".to_owned(),
                site_url: "https://t.test".to_owned(),
            }));
            let c = OpenAiClient::new(p);
            let req = CompletionRequest::simple(model_id, vec![Message::text("user", "hi")]);
            let body = c.build_request_body(&req, false).unwrap();
            assert_eq!(body["model"], serde_json::json!(model_id));
        }
    }
}
