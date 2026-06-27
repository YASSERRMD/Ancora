use serde::{Deserialize, Serialize};

use crate::types::{ContentPart, Message};

// ---- Wire types: request ---------------------------------------------------

#[derive(Debug, Serialize, Clone)]
pub(crate) struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<GeminiInlineData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<GeminiFunctionCall>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct GeminiInlineData {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct GeminiFunctionCall {
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct GeminiContent {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

/// Map a generic conversation role to the Gemini-specific role label.
///
/// Gemini accepts only `"user"` and `"model"`. OpenAI-style `"assistant"`
/// maps to `"model"`. Any other role (including `"system"`) maps to `"user"`.
pub(crate) fn map_role(role: &str) -> &'static str {
    match role {
        "assistant" | "model" => "model",
        _ => "user",
    }
}

/// Encode a `Message` into the Gemini `contents` array item shape.
///
/// Gemini uses `user` and `model` as the only valid roles; all other roles
/// (assistant, system) map to `user` as a fallback.
pub(crate) fn encode_message(msg: &Message) -> GeminiContent {
    let role = map_role(&msg.role).to_owned();
    let parts = if msg.content_parts.is_empty() {
        vec![GeminiPart { text: Some(msg.content.clone()), inline_data: None, function_call: None }]
    } else {
        msg.content_parts.iter().map(|p| match p {
            ContentPart::Text { text } => {
                GeminiPart { text: Some(text.clone()), inline_data: None, function_call: None }
            }
            ContentPart::ImageUrl { image_url } => {
                GeminiPart {
                    text: Some(format!("[image: {}]", image_url.url)),
                    inline_data: None,
                    function_call: None,
                }
            }
        }).collect()
    };
    GeminiContent { role, parts }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;

    #[test]
    fn encode_message_plain_text_produces_text_part() {
        let c = encode_message(&Message::text("user", "Hello"));
        let j = serde_json::to_value(&c).unwrap();
        assert_eq!(j["parts"][0]["text"], "Hello");
    }

    #[test]
    fn map_role_user_stays_user() {
        assert_eq!(map_role("user"), "user");
    }

    #[test]
    fn map_role_assistant_becomes_model() {
        assert_eq!(map_role("assistant"), "model");
    }

    #[test]
    fn map_role_system_falls_back_to_user() {
        assert_eq!(map_role("system"), "user");
    }

    #[test]
    fn encode_message_assistant_role_maps_to_model() {
        let c = encode_message(&Message::text("assistant", "Sure"));
        assert_eq!(c.role, "model");
    }

    #[test]
    fn encode_message_role_present() {
        let c = encode_message(&Message::text("user", "Hi"));
        let j = serde_json::to_value(&c).unwrap();
        assert!(j["role"].is_string());
    }

    #[test]
    fn encode_message_multipart_produces_array_of_parts() {
        let msg = Message::with_image("user", "describe", "https://example.com/img.jpg");
        let c = encode_message(&msg);
        assert_eq!(c.parts.len(), 2);
    }
}
