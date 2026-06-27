// OpenAI-compatible HTTP adapter for Ancora inference.
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::client::ModelClient;
use crate::error::InferenceError;
use crate::provider::ProviderProfile;
use crate::types::{
    CompletionRequest, CompletionResponse, ContentPart, FunctionCall, Message,
    TokenEvent, ToolCall, ToolDefinition,
};

// ---- Wire types -----------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WireContentPart {
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<WireImageUrl>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WireImageUrl {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct WireMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<serde_json::Value>, // String or Array
}

#[derive(Debug, Serialize, Clone)]
struct WireToolDef {
    #[serde(rename = "type")]
    kind: String,
    function: WireFunctionDef,
}

#[derive(Debug, Serialize, Clone)]
struct WireFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Clone)]
struct WireChatRequest {
    model: String,
    messages: Vec<WireMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<WireToolDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct WireResponseMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<WireToolCall>,
}

#[derive(Debug, Deserialize)]
struct WireToolCall {
    id: String,
    #[serde(rename = "type", default)]
    kind: String,
    function: WireFunctionCall,
}

#[derive(Debug, Deserialize)]
struct WireFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct WireChoice {
    message: WireResponseMessage,
}

#[derive(Debug, Default, Deserialize)]
struct WireUsage {
    #[serde(default)]
    prompt_tokens: u64,
    #[serde(default)]
    completion_tokens: u64,
}

#[derive(Debug, Deserialize)]
struct WireChatResponse {
    choices: Vec<WireChoice>,
    #[serde(default)]
    usage: WireUsage,
}

// ---- SSE streaming types --------------------------------------------------

#[derive(Debug, Deserialize)]
struct WireStreamDelta {
    #[serde(default)]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WireStreamChoice {
    delta: WireStreamDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WireStreamChunk {
    choices: Vec<WireStreamChoice>,
}

// ---- Helpers ---------------------------------------------------------------

fn encode_message(msg: &Message) -> WireMessage {
    if msg.content_parts.is_empty() {
        WireMessage {
            role: msg.role.clone(),
            content: Some(serde_json::json!(msg.content)),
        }
    } else {
        let parts: Vec<WireContentPart> = msg.content_parts.iter().map(|p| match p {
            ContentPart::Text { text } => WireContentPart {
                kind: "text".to_owned(),
                text: Some(text.clone()),
                image_url: None,
            },
            ContentPart::ImageUrl { image_url } => WireContentPart {
                kind: "image_url".to_owned(),
                text: None,
                image_url: Some(WireImageUrl {
                    url: image_url.url.clone(),
                    detail: image_url.detail.clone(),
                }),
            },
        }).collect();
        WireMessage {
            role: msg.role.clone(),
            content: Some(serde_json::json!(parts)),
        }
    }
}

fn encode_tool(t: &ToolDefinition) -> WireToolDef {
    WireToolDef {
        kind: t.kind.clone(),
        function: WireFunctionDef {
            name: t.function.name.clone(),
            description: t.function.description.clone(),
            parameters: t.function.parameters.clone(),
        },
    }
}

// ---- Client ---------------------------------------------------------------

/// HTTP client for any OpenAI-compatible endpoint, driven by a `ProviderProfile`.
pub struct OpenAiClient {
    profile: Arc<ProviderProfile>,
    region: Option<String>,
}

impl OpenAiClient {
    pub fn new(profile: Arc<ProviderProfile>) -> Self {
        Self { profile, region: None }
    }

    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    fn effective_base_url(&self) -> &str {
        self.profile.base_url_for_region(self.region.as_deref())
    }

    fn completions_url(&self) -> String {
        self.profile.completions_url(self.region.as_deref())
    }

    pub(crate) fn build_wire_request(&self, request: &CompletionRequest, stream: bool) -> WireChatRequest {
        let model_id = self.profile.resolve_model_id(&request.model_id).to_owned();
        WireChatRequest {
            model: model_id,
            messages: request.messages.iter().map(encode_message).collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            tools: request.tools.iter().map(encode_tool).collect(),
            tool_choice: request.tool_choice.clone(),
            stream,
        }
    }

    pub(crate) fn build_request_body(
        &self,
        request: &CompletionRequest,
        stream: bool,
    ) -> Result<serde_json::Value, InferenceError> {
        let wire = self.build_wire_request(request, stream);
        let mut body =
            serde_json::to_value(&wire).map_err(|e| InferenceError::Parse(e.to_string()))?;
        self.profile.request_transforms.apply(&mut body);
        Ok(body)
    }

    fn apply_auth(&self, mut req: ureq::Request) -> Result<ureq::Request, InferenceError> {
        match self.profile.auth.as_header() {
            Ok(Some((name, val))) => req = req.set(&name, &val),
            Ok(None) => {}
            Err(e) => return Err(InferenceError::MissingCredential(e)),
        }
        for (k, v) in &self.profile.extra_headers {
            req = req.set(k, v);
        }
        Ok(req)
    }

    fn post(&self, body: &serde_json::Value) -> Result<String, InferenceError> {
        let url = self.completions_url();
        let json =
            serde_json::to_string(body).map_err(|e| InferenceError::Parse(e.to_string()))?;
        let req = self.apply_auth(ureq::post(&url))?;
        req.set("Content-Type", "application/json")
            .send_string(&json)
            .map_err(|e| InferenceError::Unreachable(e.to_string()))?
            .into_string()
            .map_err(|e| InferenceError::Parse(e.to_string()))
    }

    fn post_stream(
        &self,
        body: &serde_json::Value,
        on_token: &mut dyn FnMut(TokenEvent),
    ) -> Result<(), InferenceError> {
        let url = self.completions_url();
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
            if let Some(event) = Self::parse_sse_line(&line) {
                on_token(event);
            }
        }
        Ok(())
    }

    /// Parse a single SSE data line into a `TokenEvent`.
    pub fn parse_sse_line(line: &str) -> Option<TokenEvent> {
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
        self.profile.response_transforms.apply(&mut value);
        let wire: WireChatResponse = serde_json::from_value(value)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        let msg = wire.choices.into_iter().next().map(|c| c.message).unwrap_or(WireResponseMessage {
            content: None,
            tool_calls: vec![],
        });
        let content = msg.content.unwrap_or_default();
        let tool_calls = msg
            .tool_calls
            .into_iter()
            .map(|tc| ToolCall {
                id: tc.id,
                kind: tc.kind,
                function: FunctionCall { name: tc.function.name, arguments: tc.function.arguments },
            })
            .collect();
        let tokens_in = wire.usage.prompt_tokens;
        let tokens_out = wire.usage.completion_tokens;
        let cost_usd = self
            .profile
            .model_meta(model_id)
            .and_then(|m| m.compute_cost(tokens_in, tokens_out, 0));
        Ok(CompletionResponse { content, tokens_in, tokens_out, cost_usd, tool_calls })
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
        Ok(CompletionResponse { content, tokens_in: 0, tokens_out: 0, cost_usd: None, tool_calls: vec![] })
    }
}

// ---- Tests ----------------------------------------------------------------

#[cfg(test)]
const FIXTURE_CHAT: &str = r#"{"id":"chatcmpl-abc","choices":[{"message":{"role":"assistant","content":"Hello","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":4}}"#;

#[cfg(test)]
const FIXTURE_TOOL_CALL: &str = r#"{"choices":[{"message":{"role":"assistant","content":null,"tool_calls":[{"id":"call_1","type":"function","function":{"name":"get_weather","arguments":"{\"city\":\"Paris\"}"}}]},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":20,"completion_tokens":8}}"#;

#[cfg(test)]
const FIXTURE_STREAM_LINE1: &str = r#"data: {"choices":[{"delta":{"content":"He"},"finish_reason":null}]}"#;
#[cfg(test)]
const FIXTURE_STREAM_DONE: &str = "data: [DONE]";

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
                        .with_tools()
                        .with_vision()
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
        let resp = client.parse_response(FIXTURE_CHAT, "test-model").unwrap();
        assert_eq!(resp.content, "Hello");
        assert_eq!(resp.tokens_in, 10);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn parse_stream_line_extracts_token_text() {
        let event = OpenAiClient::parse_sse_line(FIXTURE_STREAM_LINE1).unwrap();
        assert_eq!(event.text, "He");
        assert!(!event.finished);
    }

    #[test]
    fn parse_stream_done_marks_finished() {
        let event = OpenAiClient::parse_sse_line(FIXTURE_STREAM_DONE).unwrap();
        assert!(event.finished);
        assert!(event.text.is_empty());
    }

    #[test]
    fn alias_resolved_in_request_body() {
        let client = test_client();
        let req = CompletionRequest::simple("tm", vec![]);
        let body = client.build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], serde_json::json!("test-model"));
    }

    #[test]
    fn cost_usd_computed_from_pricing_metadata() {
        let client = test_client();
        let resp = client.parse_response(FIXTURE_CHAT, "test-model").unwrap();
        assert!(resp.cost_usd.is_some());
        assert!(resp.cost_usd.unwrap() > 0.0);
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
        let resp = client.parse_response(FIXTURE_CHAT, "expensive-model").unwrap();
        let expected = 10.0 * 30.0 / 1_000_000.0 + 4.0 * 60.0 / 1_000_000.0;
        let cost = resp.cost_usd.expect("cost should be Some for priced model");
        assert!((cost - expected).abs() < 1e-12);
    }

    #[test]
    fn no_pricing_metadata_yields_none_cost() {
        let profile = Arc::new(
            ProviderProfile::new("unpriced", "http://localhost", AuthStrategy::None)
                .add_model(ModelMeta::new("free-model", 4_096)),
        );
        let client = OpenAiClient::new(profile);
        let resp = client.parse_response(FIXTURE_CHAT, "free-model").unwrap();
        assert!(resp.cost_usd.is_none());
    }

    #[test]
    fn tool_call_mapping_round_trip() {
        let client = test_client();
        let resp = client.parse_response(FIXTURE_TOOL_CALL, "test-model").unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        let tc = &resp.tool_calls[0];
        assert_eq!(tc.id, "call_1");
        assert_eq!(tc.function.name, "get_weather");
        assert!(tc.function.arguments.contains("Paris"));
    }
}
