use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::client::ModelClient;
use crate::error::InferenceError;
use crate::provider::ProviderProfile;
use crate::types::{
    CompletionRequest, CompletionResponse, ContentPart, FunctionCall, Message, TokenEvent,
    ToolCall, ToolDefinition,
};

// ---- Wire types: request ---------------------------------------------------

#[derive(Debug, Serialize, Clone)]
pub(crate) struct AnthropicRequestMessage {
    pub role: String,
    pub content: serde_json::Value,
}

// ---- Wire types: request body ----------------------------------------------

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<AnthropicRequestMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<AnthropicToolDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

// ---- Wire types: response body ---------------------------------------------

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u64,
    output_tokens: u64,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseBlock>,
    usage: AnthropicUsage,
}

// ---- Wire types: streaming -------------------------------------------------

/// Subset of Anthropic SSE event types relevant to token streaming.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicStreamEvent {
    ContentBlockDelta { delta: AnthropicDelta },
    MessageDelta,
    MessageStop,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct AnthropicDelta {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    text: String,
}

// ---- Wire types: tool definitions -----------------------------------------

#[derive(Debug, Serialize, Clone)]
pub(crate) struct AnthropicToolDef {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

// ---- Wire types: response tool_use block ----------------------------------

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum AnthropicResponseBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: serde_json::Value },
    #[serde(other)]
    Unknown,
}

// ---- Encoding helpers ------------------------------------------------------

/// Encode a `ToolDefinition` into the Anthropic tools array format.
///
/// Anthropic uses `input_schema` (not `parameters`) for the JSON Schema.
pub(crate) fn encode_tool(t: &ToolDefinition) -> AnthropicToolDef {
    AnthropicToolDef {
        name: t.function.name.clone(),
        description: t.function.description.clone(),
        input_schema: t.function.parameters.clone(),
    }
}

/// Decode a list of Anthropic response content blocks into `ToolCall`s.
pub(crate) fn decode_tool_calls(blocks: Vec<AnthropicResponseBlock>) -> (String, Vec<ToolCall>) {
    let mut text = String::new();
    let mut calls = Vec::new();
    for block in blocks {
        match block {
            AnthropicResponseBlock::Text { text: t } => text.push_str(&t),
            AnthropicResponseBlock::ToolUse { id, name, input } => {
                calls.push(ToolCall {
                    id,
                    kind: "function".to_owned(),
                    function: FunctionCall {
                        name,
                        arguments: serde_json::to_string(&input).unwrap_or_default(),
                    },
                });
            }
            AnthropicResponseBlock::Unknown => {}
        }
    }
    (text, calls)
}

/// Parse a single `data: ...` SSE line from an Anthropic streaming response.
///
/// `content_block_delta` events with `text_delta` type emit token text.
/// `message_delta` and `message_stop` events emit a finished sentinel.
pub fn parse_sse_line(line: &str) -> Option<TokenEvent> {
    let data = line.strip_prefix("data: ")?;
    if data.trim().is_empty() {
        return None;
    }
    let event: AnthropicStreamEvent = serde_json::from_str(data).ok()?;
    match event {
        AnthropicStreamEvent::ContentBlockDelta { delta } if delta.kind == "text_delta" => {
            Some(TokenEvent { text: delta.text, finished: false })
        }
        AnthropicStreamEvent::MessageDelta | AnthropicStreamEvent::MessageStop => {
            Some(TokenEvent { text: String::new(), finished: true })
        }
        _ => None,
    }
}

/// Separate the optional system message from the rest of a message list.
///
/// Anthropic puts the system prompt at the top level of the request body,
/// not inside the `messages` array. The first `role=="system"` message is
/// extracted; remaining messages are returned in order.
pub(crate) fn extract_system(messages: &[Message]) -> (Option<String>, Vec<&Message>) {
    let system = messages.iter().find(|m| m.role == "system").map(|m| m.content.clone());
    let rest: Vec<&Message> = messages.iter().filter(|m| m.role != "system").collect();
    (system, rest)
}

/// Encode a `Message` into the Anthropic wire message shape.
///
/// Plain-text messages serialize content as a JSON string.
/// Messages with content parts serialize as a JSON array of blocks.
/// Tool-result messages (`role == "tool"`) are re-wrapped as a `user`
/// message containing a `tool_result` content block.
pub(crate) fn encode_message(msg: &Message) -> AnthropicRequestMessage {
    if msg.role == "tool" {
        let block = serde_json::json!([{
            "type": "tool_result",
            "tool_use_id": "",
            "content": msg.content
        }]);
        return AnthropicRequestMessage { role: "user".to_owned(), content: block };
    }
    if msg.content_parts.is_empty() {
        AnthropicRequestMessage {
            role: msg.role.clone(),
            content: serde_json::json!(msg.content),
        }
    } else {
        let parts: Vec<serde_json::Value> = msg.content_parts.iter().map(|p| match p {
            ContentPart::Text { text } => serde_json::json!({"type": "text", "text": text}),
            ContentPart::ImageUrl { image_url } => {
                // data: URLs are base64-encoded; Anthropic accepts them via the
                // "base64" source type. Plain URLs use the "url" source type.
                if let Some(rest) = image_url.url.strip_prefix("data:") {
                    if let Some(idx) = rest.find(";base64,") {
                        let media_type = &rest[..idx];
                        let data = &rest[idx + 8..];
                        return serde_json::json!({
                            "type": "image",
                            "source": {"type": "base64", "media_type": media_type, "data": data}
                        });
                    }
                }
                serde_json::json!({
                    "type": "image",
                    "source": {"type": "url", "url": image_url.url}
                })
            }
        }).collect();
        AnthropicRequestMessage {
            role: msg.role.clone(),
            content: serde_json::json!(parts),
        }
    }
}

// ---- Client ----------------------------------------------------------------

/// HTTP client for the Anthropic Messages API.
///
/// Wire format differs from OpenAI: system prompt is a top-level field,
/// tools use `input_schema` instead of `parameters`, and responses carry
/// a `content` array of typed blocks rather than a single `message` choice.
pub struct AnthropicClient {
    profile: Arc<ProviderProfile>,
}

impl AnthropicClient {
    pub fn new(profile: Arc<ProviderProfile>) -> Self {
        Self { profile }
    }

    /// Build the JSON request body for a (non-)streaming Anthropic call.
    pub(crate) fn build_request_body(
        &self,
        request: &CompletionRequest,
        stream: bool,
    ) -> Result<serde_json::Value, InferenceError> {
        let model_id = self.profile.resolve_model_id(&request.model_id).to_owned();
        let (system, non_system) = extract_system(&request.messages);
        let messages: Vec<AnthropicRequestMessage> =
            non_system.iter().map(|m| encode_message(m)).collect();
        let tools: Vec<AnthropicToolDef> = request.tools.iter().map(encode_tool).collect();
        let wire = AnthropicRequest {
            model: model_id,
            max_tokens: request.max_tokens.unwrap_or(4096),
            system,
            messages,
            tools,
            stream: if stream { Some(true) } else { None },
        };
        let mut body = serde_json::to_value(&wire)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
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
        let url = self.profile.completions_url(None);
        let json = serde_json::to_string(body)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
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
        let url = self.profile.completions_url(None);
        let json = serde_json::to_string(body)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        let req = self.apply_auth(ureq::post(&url))?;
        let resp = req
            .set("Content-Type", "application/json")
            .send_string(&json)
            .map_err(|e| InferenceError::Unreachable(e.to_string()))?;
        use std::io::BufRead;
        let reader = std::io::BufReader::new(resp.into_reader());
        for line in reader.lines() {
            let line = line.map_err(|e| InferenceError::Parse(e.to_string()))?;
            if let Some(event) = parse_sse_line(&line) {
                on_token(event);
            }
        }
        Ok(())
    }

    /// Decode a recorded Anthropic JSON response body.
    ///
    /// Combines text blocks into `content`, converts `tool_use` blocks
    /// into `ToolCall`s, and computes cost from the profile's pricing metadata.
    pub fn parse_response(
        &self,
        body: &str,
        model_id: &str,
    ) -> Result<CompletionResponse, InferenceError> {
        let wire: AnthropicResponse = serde_json::from_str(body)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        let (content, tool_calls) = decode_tool_calls(wire.content);
        let tokens_in = wire.usage.input_tokens;
        let tokens_out = wire.usage.output_tokens;
        let cost_usd = self
            .profile
            .model_meta(model_id)
            .and_then(|m| m.compute_cost(tokens_in, tokens_out, 0));
        Ok(CompletionResponse { content, tokens_in, tokens_out, cost_usd, tool_calls })
    }
}

impl ModelClient for AnthropicClient {
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
        Ok(CompletionResponse {
            content,
            tokens_in: 0,
            tokens_out: 0,
            cost_usd: None,
            tool_calls: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;

    #[test]
    fn encode_message_tool_role_becomes_user_with_tool_result_block() {
        let msg = Message::text("tool", "sunny in Paris");
        let m = encode_message(&msg);
        let j = serde_json::to_value(&m).unwrap();
        assert_eq!(j["role"], "user");
        let block = &j["content"][0];
        assert_eq!(block["type"], "tool_result");
        assert_eq!(block["content"], "sunny in Paris");
    }

    #[test]
    fn encode_message_plain_text_content_is_string() {
        let m = encode_message(&Message::text("user", "Hello"));
        let j = serde_json::to_value(&m).unwrap();
        assert_eq!(j["role"], "user");
        assert_eq!(j["content"], "Hello");
    }

    #[test]
    fn encode_message_preserves_assistant_role() {
        let m = encode_message(&Message::text("assistant", "Hi"));
        let j = serde_json::to_value(&m).unwrap();
        assert_eq!(j["role"], "assistant");
    }

    #[test]
    fn parse_sse_text_delta_emits_token_text() {
        let line = r#"data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}"#;
        let ev = parse_sse_line(line).unwrap();
        assert_eq!(ev.text, "Hello");
        assert!(!ev.finished);
    }

    #[test]
    fn parse_sse_message_stop_emits_finished_sentinel() {
        let line = r#"data: {"type":"message_stop"}"#;
        let ev = parse_sse_line(line).unwrap();
        assert!(ev.finished);
        assert!(ev.text.is_empty());
    }

    #[test]
    fn parse_sse_message_delta_emits_finished_sentinel() {
        let line = r#"data: {"type":"message_delta","delta":{"stop_reason":"end_turn"},"usage":{"output_tokens":5}}"#;
        let ev = parse_sse_line(line).unwrap();
        assert!(ev.finished);
    }

    #[test]
    fn parse_sse_other_event_returns_none() {
        let line = r#"data: {"type":"message_start","message":{"id":"msg_1"}}"#;
        assert!(parse_sse_line(line).is_none());
    }

    #[test]
    fn encode_tool_maps_name_description_and_schema() {
        use crate::types::{FunctionDefinition, ToolDefinition};
        let td = ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: "get_weather".to_owned(),
                description: "Get weather".to_owned(),
                parameters: serde_json::json!({"type": "object"}),
            },
        };
        let at = encode_tool(&td);
        assert_eq!(at.name, "get_weather");
        assert_eq!(at.input_schema, serde_json::json!({"type": "object"}));
    }

    #[test]
    fn decode_tool_calls_extracts_tool_use_block() {
        let blocks = vec![
            AnthropicResponseBlock::ToolUse {
                id: "toolu_01".to_owned(),
                name: "get_weather".to_owned(),
                input: serde_json::json!({"city": "Paris"}),
            },
        ];
        let (text, calls) = decode_tool_calls(blocks);
        assert!(text.is_empty());
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id, "toolu_01");
        assert_eq!(calls[0].function.name, "get_weather");
        assert!(calls[0].function.arguments.contains("Paris"));
    }

    #[test]
    fn decode_tool_calls_collects_text_blocks() {
        let blocks = vec![
            AnthropicResponseBlock::Text { text: "hello".to_owned() },
            AnthropicResponseBlock::Text { text: " world".to_owned() },
        ];
        let (text, calls) = decode_tool_calls(blocks);
        assert_eq!(text, "hello world");
        assert!(calls.is_empty());
    }

    #[test]
    fn extract_system_returns_system_text_and_remainder() {
        let msgs = vec![
            Message::text("system", "Be helpful"),
            Message::text("user", "Hi"),
        ];
        let (sys, rest) = extract_system(&msgs);
        assert_eq!(sys.as_deref(), Some("Be helpful"));
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].role, "user");
    }

    #[test]
    fn extract_system_none_when_no_system_message() {
        let msgs = vec![Message::text("user", "Hello")];
        let (sys, rest) = extract_system(&msgs);
        assert!(sys.is_none());
        assert_eq!(rest.len(), 1);
    }

    #[test]
    fn encode_message_multipart_content_is_array() {
        let msg = Message::with_image("user", "describe this", "https://example.com/img.png");
        let m = encode_message(&msg);
        let j = serde_json::to_value(&m).unwrap();
        assert!(j["content"].is_array());
    }

    #[test]
    fn encode_message_url_image_produces_image_block_with_url_source() {
        let msg = Message::with_image("user", "look", "https://example.com/photo.jpg");
        let m = encode_message(&msg);
        let j = serde_json::to_value(&m).unwrap();
        let img_block = &j["content"][1];
        assert_eq!(img_block["type"], "image");
        assert_eq!(img_block["source"]["type"], "url");
        assert_eq!(img_block["source"]["url"], "https://example.com/photo.jpg");
    }

    #[test]
    fn encode_message_data_url_image_produces_base64_source() {
        use crate::types::{ContentPart, ImageUrl};
        let msg = Message {
            role: "user".to_owned(),
            content: String::new(),
            content_parts: vec![ContentPart::ImageUrl {
                image_url: ImageUrl {
                    url: "data:image/jpeg;base64,/9j/abc".to_owned(),
                    detail: None,
                },
            }],
        };
        let m = encode_message(&msg);
        let j = serde_json::to_value(&m).unwrap();
        let block = &j["content"][0];
        assert_eq!(block["source"]["type"], "base64");
        assert_eq!(block["source"]["media_type"], "image/jpeg");
        assert_eq!(block["source"]["data"], "/9j/abc");
    }
}
