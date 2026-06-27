use serde::Serialize;

use crate::types::{ContentPart, Message};

// ---- Wire types: request ---------------------------------------------------

#[derive(Debug, Serialize, Clone)]
pub(crate) struct AnthropicRequestMessage {
    pub role: String,
    pub content: serde_json::Value,
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
