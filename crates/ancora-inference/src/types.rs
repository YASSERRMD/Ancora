use serde::{Deserialize, Serialize};

/// A single content part in a message (text or image URL).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

/// An image URL content block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// A tool (function) the model may call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: FunctionDefinition,
}

/// The schema for a callable function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// A tool call returned by the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: FunctionCall,
}

/// The function name and arguments from a model tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// A single turn in a conversation, identified by role.
#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    /// Plain-text content (used when `content_parts` is empty).
    pub content: String,
    /// Rich content parts for vision (image URLs) and multi-modal inputs.
    /// When non-empty, overrides `content` in the wire format.
    pub content_parts: Vec<ContentPart>,
}

impl Message {
    /// Convenience constructor for a plain-text message.
    pub fn text(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self { role: role.into(), content: content.into(), content_parts: vec![] }
    }

    /// Convenience constructor for a vision message with an image URL.
    pub fn with_image(role: impl Into<String>, text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: String::new(),
            content_parts: vec![
                ContentPart::Text { text: text.into() },
                ContentPart::ImageUrl {
                    image_url: ImageUrl { url: url.into(), detail: None },
                },
            ],
        }
    }
}

/// A request sent to a model for completion.
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub model_id: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    /// Tool definitions available to the model.
    pub tools: Vec<ToolDefinition>,
    /// Tool choice hint: `None` (default), `"auto"`, `"none"`, or a specific tool name.
    pub tool_choice: Option<String>,
}

impl CompletionRequest {
    pub fn simple(model_id: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model_id: model_id.into(),
            messages,
            max_tokens: None,
            temperature: None,
            tools: vec![],
            tool_choice: None,
        }
    }
}

/// The response returned by a model after completion.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionResponse {
    pub content: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
    /// USD cost computed from provider pricing metadata, when available.
    pub cost_usd: Option<f64>,
    /// Tool calls requested by the model (empty for non-tool responses).
    pub tool_calls: Vec<ToolCall>,
}

/// A token fragment emitted during streaming completion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenEvent {
    pub text: String,
    pub finished: bool,
}
