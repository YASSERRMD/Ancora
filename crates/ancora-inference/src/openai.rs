// OpenAI-compatible HTTP adapter for Ancora inference.
use serde::{Deserialize, Serialize};

use crate::client::ModelClient;
use crate::error::InferenceError;
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

/// HTTP client for any OpenAI-compatible endpoint (OpenAI, Ollama, vLLM, llama.cpp).
pub struct OpenAiClient {
    base_url: String,
}

impl OpenAiClient {
    /// Create a client pointing at `base_url` (e.g. `http://localhost:11434` for Ollama).
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into() }
    }

    fn map_messages(messages: &[Message]) -> Vec<WireMessage> {
        messages.iter().map(|m| WireMessage { role: m.role.clone(), content: m.content.clone() }).collect()
    }

    fn post(&self, body: &ChatCompletionRequest) -> Result<String, InferenceError> {
        let url = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));
        let json = serde_json::to_string(body).map_err(|e| InferenceError::Parse(e.to_string()))?;
        ureq::post(&url)
            .set("Content-Type", "application/json")
            .send_string(&json)
            .map_err(|e| InferenceError::Unreachable(e.to_string()))?
            .into_string()
            .map_err(|e| InferenceError::Parse(e.to_string()))
    }

    fn post_stream(
        &self,
        body: &ChatCompletionRequest,
        on_token: &mut dyn FnMut(TokenEvent),
    ) -> Result<(), InferenceError> {
        let url = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));
        let json = serde_json::to_string(body).map_err(|e| InferenceError::Parse(e.to_string()))?;
        let resp = ureq::post(&url)
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

    pub fn parse_response(body: &str) -> Result<CompletionResponse, InferenceError> {
        let wire: ChatCompletionResponse = serde_json::from_str(body)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        let content = wire.choices.into_iter().next()
            .map(|c| c.message.content)
            .unwrap_or_default();
        Ok(CompletionResponse {
            content,
            tokens_in: wire.usage.prompt_tokens,
            tokens_out: wire.usage.completion_tokens,
        })
    }
}

impl ModelClient for OpenAiClient {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, InferenceError> {
        let wire_req = ChatCompletionRequest {
            model: &request.model_id,
            messages: Self::map_messages(&request.messages),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            tools: vec![],
            stream: false,
        };
        let body = self.post(&wire_req)?;
        Self::parse_response(&body)
    }

    fn stream_complete(
        &self,
        request: &CompletionRequest,
        on_token: &mut dyn FnMut(TokenEvent),
    ) -> Result<CompletionResponse, InferenceError> {
        let wire_req = ChatCompletionRequest {
            model: &request.model_id,
            messages: Self::map_messages(&request.messages),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            tools: vec![],
            stream: true,
        };
        let mut content = String::new();
        self.post_stream(&wire_req, &mut |event: TokenEvent| {
            if !event.text.is_empty() {
                content.push_str(&event.text);
            }
            on_token(event);
        })?;
        Ok(CompletionResponse { content, tokens_in: 0, tokens_out: 0 })
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<WireMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<serde_json::Value>,
    stream: bool,
}

#[cfg(test)]
const FIXTURE_CHAT_RESPONSE: &str = r#"{"id":"chatcmpl-abc","object":"chat.completion","choices":[{"index":0,"message":{"role":"assistant","content":"Hello from Ollama"},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":4}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_response_parses_fixture() {
        let resp = OpenAiClient::parse_response(FIXTURE_CHAT_RESPONSE).unwrap();
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
}
