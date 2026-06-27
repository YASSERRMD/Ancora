use serde::Serialize;

use crate::types::{ContentPart, Message};

// ---- Wire types: request ---------------------------------------------------

#[derive(Debug, Serialize, Clone)]
pub(crate) struct AnthropicRequestMessage {
    pub role: String,
    pub content: serde_json::Value,
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
    fn encode_message_multipart_content_is_array() {
        let msg = Message::with_image("user", "describe this", "https://example.com/img.png");
        let m = encode_message(&msg);
        let j = serde_json::to_value(&m).unwrap();
        assert!(j["content"].is_array());
    }
}
