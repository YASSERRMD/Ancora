use serde::{Deserialize, Serialize};

use crate::types::{ContentPart, FunctionCall, Message, ToolCall, ToolDefinition};

// ---- Wire types: request ---------------------------------------------------

#[derive(Debug, Serialize, Clone)]
pub(crate) struct AnthropicRequestMessage {
    pub role: String,
    pub content: serde_json::Value,
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
pub(crate) fn encode_message(msg: &Message) -> AnthropicRequestMessage {
    if msg.content_parts.is_empty() {
        AnthropicRequestMessage {
            role: msg.role.clone(),
            content: serde_json::json!(msg.content),
        }
    } else {
        let parts: Vec<serde_json::Value> = msg.content_parts.iter().map(|p| match p {
            ContentPart::Text { text } => serde_json::json!({"type": "text", "text": text}),
            ContentPart::ImageUrl { image_url } => {
                serde_json::json!({"type": "text", "text": format!("[image: {}]", image_url.url)})
            }
        }).collect();
        AnthropicRequestMessage {
            role: msg.role.clone(),
            content: serde_json::json!(parts),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;

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
}
