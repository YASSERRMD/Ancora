// OpenAI-compatible HTTP adapter for Ancora inference.
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::client::ModelClient;
use crate::error::InferenceError;
use crate::provider::ProviderProfile;
use crate::types::{CompletionRequest, CompletionResponse, Message, TokenEvent};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct WireMessage {
    pub role: String,
    pub content: String,
}

/// A tool definition sent in the request to tell the model which functions it may call.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// A tool call returned by the model in a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: WireMessage,
}

#[derive(Debug, Default, Deserialize)]
struct Usage {
    #[serde(default)]
    prompt_tokens: u64,
    #[serde(default)]
    completion_tokens: u64,
}

#[derive(Debug, Deserialize)]
struct WireDelta {
    #[serde(default)]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WireStreamChoice {
    delta: WireDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WireStreamChunk {
    choices: Vec<WireStreamChoice>,
}

/// HTTP client for any OpenAI-compatible endpoint, driven by a `ProviderProfile`.
///
/// The profile supplies the base URL, authentication, and optional request/response
/// transforms. An optional region label selects a regional base-URL override.
pub struct OpenAiClient {
    profile: Arc<ProviderProfile>,
    region: Option<String>,
}

impl OpenAiClient {
    /// Create a client for the given provider profile.
    pub fn new(profile: Arc<ProviderProfile>) -> Self {
        Self { profile, region: None }
    }

    /// Select a regional base-URL override defined in the profile.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    fn effective_base_url(&self) -> &str {
        self.profile.base_url_for_region(self.region.as_deref())
    }

    fn map_messages(messages: &[Message]) -> Vec<WireMessage> {
        messages
            .iter()
            .map(|m| WireMessage { role: m.role.clone(), content: m.content.clone() })
            .collect()
    }

    fn build_request_body(
        &self,
        request: &CompletionRequest,
        stream: bool,
    ) -> Result<serde_json::Value, InferenceError> {
        let model_id = self.profile.resolve_model_id(&request.model_id);
        let mut body = serde_json::json!({
            "model": model_id,
            "messages": Self::map_messages(&request.messages),
            "stream": stream,
        });
        if let Some(mt) = request.max_tokens {
            body["max_tokens"] = serde_json::json!(mt);
        }
        if let Some(t) = request.temperature {
            body["temperature"] = serde_json::json!(t);
        }
        // Apply provider-specific request transforms.
        self.profile.request_transforms.apply(&mut body);
        Ok(body)
    }

    fn apply_auth(&self, req: ureq::Request) -> Result<ureq::Request, InferenceError> {
        match self.profile.auth.as_header() {
            Ok(Some((name, val))) => Ok(req.set(&name, &val)),
            Ok(None) => Ok(req),
            Err(e) => Err(InferenceError::MissingCredential(e)),
        }
    }

    fn post(&self, body: &serde_json::Value) -> Result<String, InferenceError> {
        let url = format!(
            "{}/v1/chat/completions",
            self.effective_base_url().trim_end_matches('/')
        );
        let json =
            serde_json::to_string(body).map_err(|e| InferenceError::Parse(e.to_string()))?;
        let req = self.apply_auth(ureq::post(&url))?;
        let resp = req
            .set("Content-Type", "application/json")
            .send_string(&json)
            .map_err(|e| {
                let msg = e.to_string();
                if msg.contains("401") || msg.contains("403") {
                    InferenceError::from_http(401, &msg, None)
                } else if msg.contains("429") {
                    InferenceError::from_http(429, &msg, None)
                } else {
                    InferenceError::Unreachable(msg)
                }
            })?;
        resp.into_string().map_err(|e| InferenceError::Parse(e.to_string()))
    }

    fn post_stream(
        &self,
        body: &serde_json::Value,
        on_token: &mut dyn FnMut(TokenEvent),
    ) -> Result<(), InferenceError> {
        let url = format!(
            "{}/v1/chat/completions",
            self.effective_base_url().trim_end_matches('/')
        );
        let json =
            serde_json::to_string(body).map_err(|e| InferenceError::Parse(e.to_string()))?;
        let req = self.apply_auth(ureq::post(&url))?;
        let resp = req
            .set("Content-Type", "application/json")
            .send_string(&json)
            .map_err(|e| InferenceError::Unreachable(e.to_string()))?;
        use std::io::BufRead;
        let reader = std::io::BufReader::new(resp.into_reader());
        for line in reader.lines() {
            let line = line.map_err(|e| InferenceError::Parse(e.to_string()))?;
            if let Some(event) = Self::parse_stream_chunk(&line) {
                on_token(event);
            }
        }
        Ok(())
    }

    pub(crate) fn parse_stream_chunk(line: &str) -> Option<TokenEvent> {
        let data = line.strip_prefix("data: ")?;
        if data.trim() == "[DONE]" {
            return Some(TokenEvent { text: String::new(), finished: true });
        }
        let chunk: WireStreamChunk = serde_json::from_str(data).ok()?;
        let choice = chunk.choices.into_iter().next()?;
        let finished = choice.finish_reason.is_some();
        let text = choice.delta.content.unwrap_or_default();
        Some(TokenEvent { text, finished })
    }

    pub fn parse_response(
        &self,
        body: &str,
        model_id: &str,
    ) -> Result<CompletionResponse, InferenceError> {
        let mut value: serde_json::Value = serde_json::from_str(body)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        // Apply response transforms before deserialising.
        self.profile.response_transforms.apply(&mut value);
        let wire: ChatCompletionResponse = serde_json::from_value(value)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        let content = wire.choices.into_iter().next().map(|c| c.message.content).unwrap_or_default();
        let tokens_in = wire.usage.prompt_tokens;
        let tokens_out = wire.usage.completion_tokens;
        let cost_usd = self
            .profile
            .model_meta(model_id)
            .and_then(|m| m.compute_cost(tokens_in, tokens_out, 0));
        Ok(CompletionResponse { content, tokens_in, tokens_out, cost_usd })
    }
}

impl ModelClient for OpenAiClient {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, InferenceError> {
        let body = self.build_request_body(request, false)?;
        let resp_str = self.post(&body)?;
        self.parse_response(&resp_str, &request.model_id)
    }

    fn stream_complete(
        &self,
        request: &CompletionRequest,
        on_token: &mut dyn FnMut(TokenEvent),
    ) -> Result<CompletionResponse, InferenceError> {
        let body = self.build_request_body(request, true)?;
        let mut content = String::new();
        self.post_stream(&body, &mut |event: TokenEvent| {
            if !event.text.is_empty() {
                content.push_str(&event.text);
            }
            on_token(event);
        })?;
        Ok(CompletionResponse { content, tokens_in: 0, tokens_out: 0, cost_usd: None })
    }
}

#[cfg(test)]
const FIXTURE_CHAT_RESPONSE: &str = r#"{"id":"chatcmpl-abc","object":"chat.completion","choices":[{"index":0,"message":{"role":"assistant","content":"Hello from Ollama"},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":4}}"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

    fn test_profile() -> Arc<ProviderProfile> {
        Arc::new(
            ProviderProfile::new("test", "http://localhost:11434", AuthStrategy::None)
                .add_model(
                    ModelMeta::new("test-model", 4096)
                        .with_pricing(1.0, 2.0)
                        .with_streaming(),
                )
                .add_alias("tm", "test-model"),
        )
    }

    fn test_client() -> OpenAiClient {
        OpenAiClient::new(test_profile())
    }

    #[test]
    fn parse_response_parses_fixture() {
        let client = test_client();
        let resp = client.parse_response(FIXTURE_CHAT_RESPONSE, "test-model").unwrap();
        assert_eq!(resp.content, "Hello from Ollama");
        assert_eq!(resp.tokens_in, 10);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn parse_stream_chunk_extracts_token_text() {
        let line = r#"data: {"choices":[{"delta":{"content":"Hi"},"finish_reason":null}]}"#;
        let event = OpenAiClient::parse_stream_chunk(line).unwrap();
        assert_eq!(event.text, "Hi");
        assert!(!event.finished);
    }

    #[test]
    fn parse_stream_chunk_done_sentinel_marks_finished() {
        let event = OpenAiClient::parse_stream_chunk("data: [DONE]").unwrap();
        assert!(event.finished);
        assert!(event.text.is_empty());
    }

    #[test]
    fn alias_resolved_in_request_body() {
        let client = test_client();
        let req = crate::types::CompletionRequest {
            model_id: "tm".to_owned(),
            messages: vec![],
            max_tokens: None,
            temperature: None,
        };
        let body = client.build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], serde_json::json!("test-model"));
    }

    #[test]
    fn cost_usd_computed_from_pricing_metadata() {
        let client = test_client();
        let resp = client.parse_response(FIXTURE_CHAT_RESPONSE, "test-model").unwrap();
        // 10 tokens_in @ $1/M + 4 tokens_out @ $2/M = very small but non-None
        assert!(resp.cost_usd.is_some());
        let cost = resp.cost_usd.unwrap();
        assert!(cost > 0.0);
    }

    #[test]
    fn regional_url_used_when_region_set() {
        let profile = Arc::new(
            ProviderProfile::new("reg", "https://default.api.test", AuthStrategy::None)
                .add_region("eu", "https://eu.api.test"),
        );
        let client = OpenAiClient::new(profile).with_region("eu");
        assert_eq!(client.effective_base_url(), "https://eu.api.test");
    }

    #[test]
    fn pricing_metadata_feeds_cost_accounting() {
        let profile = Arc::new(
            ProviderProfile::new("pricing-test", "http://localhost", AuthStrategy::None)
                .add_model(
                    ModelMeta::new("expensive-model", 32_000)
                        .with_pricing(30.0, 60.0),
                ),
        );
        let client = OpenAiClient::new(profile);
        // 1000 input tokens @ $30/M + 500 output tokens @ $60/M
        // = 0.030 + 0.030 = $0.060
        let resp = client.parse_response(FIXTURE_CHAT_RESPONSE, "expensive-model").unwrap();
        // FIXTURE has 10 prompt + 4 completion tokens
        let expected = 10.0 * 30.0 / 1_000_000.0 + 4.0 * 60.0 / 1_000_000.0;
        let cost = resp.cost_usd.expect("cost should be Some for priced model");
        assert!((cost - expected).abs() < 1e-12, "cost {cost} != {expected}");
    }

    #[test]
    fn no_pricing_metadata_yields_none_cost() {
        let profile = Arc::new(
            ProviderProfile::new("unpriced", "http://localhost", AuthStrategy::None)
                .add_model(ModelMeta::new("free-model", 4_096)),
        );
        let client = OpenAiClient::new(profile);
        let resp = client.parse_response(FIXTURE_CHAT_RESPONSE, "free-model").unwrap();
        assert!(resp.cost_usd.is_none());
    }
}
