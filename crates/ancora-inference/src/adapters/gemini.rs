use serde::{Deserialize, Serialize};

use crate::types::{ContentPart, FunctionCall, Message, TokenEvent, ToolCall, ToolDefinition};

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

// ---- Wire types: tool definitions -----------------------------------------

#[derive(Debug, Serialize, Clone)]
pub(crate) struct GeminiFunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct GeminiTool {
    pub function_declarations: Vec<GeminiFunctionDeclaration>,
}

// ---- Wire types: response --------------------------------------------------

#[derive(Debug, Deserialize)]
pub(crate) struct GeminiResponsePart {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub function_call: Option<GeminiFunctionCall>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GeminiResponseContent {
    pub parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeminiCandidate {
    pub content: GeminiResponseContent,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeminiUsageMetadata {
    #[serde(default)]
    pub prompt_token_count: u64,
    #[serde(default)]
    pub candidates_token_count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    pub usage_metadata: GeminiUsageMetadata,
}

/// Encode a `ToolDefinition` into a Gemini `functionDeclarations` entry.
///
/// Gemini uses `parameters` (same key as the JSON Schema spec), unlike
/// Anthropic which uses `input_schema`.
pub(crate) fn encode_tool(t: &ToolDefinition) -> GeminiFunctionDeclaration {
    GeminiFunctionDeclaration {
        name: t.function.name.clone(),
        description: t.function.description.clone(),
        parameters: t.function.parameters.clone(),
    }
}

/// Decode Gemini response candidates into text and tool calls.
pub(crate) fn decode_response(resp: GeminiResponse) -> (String, Vec<ToolCall>, u64, u64) {
    let tokens_in = resp.usage_metadata.prompt_token_count;
    let tokens_out = resp.usage_metadata.candidates_token_count;
    let candidate = match resp.candidates.into_iter().next() {
        Some(c) => c,
        None => return (String::new(), vec![], tokens_in, tokens_out),
    };
    let mut text = String::new();
    let mut calls = Vec::new();
    for part in candidate.content.parts {
        if let Some(t) = part.text {
            text.push_str(&t);
        }
        if let Some(fc) = part.function_call {
            calls.push(ToolCall {
                id: format!("call_{}", fc.name),
                kind: "function".to_owned(),
                function: FunctionCall {
                    name: fc.name.clone(),
                    arguments: serde_json::to_string(&fc.args).unwrap_or_default(),
                },
            });
        }
    }
    (text, calls, tokens_in, tokens_out)
}

/// Parse a single `data: ...` SSE line from a Gemini streaming response.
///
/// Each chunk is a full `GeminiResponse` JSON object. Text parts in the
/// first candidate are concatenated into the `TokenEvent.text`. The
/// event is marked finished when the candidate carries a non-null
/// `finishReason`.
pub fn parse_sse_line(line: &str) -> Option<TokenEvent> {
    let data = line.strip_prefix("data: ")?;
    if data.trim().is_empty() {
        return None;
    }
    let resp: GeminiResponse = serde_json::from_str(data).ok()?;
    let candidate = resp.candidates.into_iter().next()?;
    let finished = candidate.finish_reason.is_some();
    let text: String = candidate.content.parts.into_iter()
        .filter_map(|p| p.text)
        .collect();
    Some(TokenEvent { text, finished })
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
    fn parse_sse_text_chunk_yields_token_text() {
        let line = r#"data: {"candidates":[{"content":{"role":"model","parts":[{"text":"Hello"}]},"finishReason":null}],"usageMetadata":{"promptTokenCount":5,"candidatesTokenCount":0}}"#;
        let ev = parse_sse_line(line).unwrap();
        assert_eq!(ev.text, "Hello");
        assert!(!ev.finished);
    }

    #[test]
    fn parse_sse_stop_chunk_marks_finished() {
        let line = r#"data: {"candidates":[{"content":{"role":"model","parts":[{"text":" world"}]},"finishReason":"STOP"}],"usageMetadata":{"promptTokenCount":5,"candidatesTokenCount":2}}"#;
        let ev = parse_sse_line(line).unwrap();
        assert_eq!(ev.text, " world");
        assert!(ev.finished);
    }

    #[test]
    fn parse_sse_empty_data_returns_none() {
        assert!(parse_sse_line("data: ").is_none());
    }

    #[test]
    fn encode_tool_preserves_name_description_parameters() {
        use crate::types::{FunctionDefinition, ToolDefinition};
        let td = ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: "get_weather".to_owned(),
                description: "Get weather".to_owned(),
                parameters: serde_json::json!({"type": "object"}),
            },
        };
        let fd = encode_tool(&td);
        assert_eq!(fd.name, "get_weather");
        assert_eq!(fd.parameters, serde_json::json!({"type": "object"}));
    }

    #[test]
    fn decode_response_extracts_function_call_part() {
        let resp = GeminiResponse {
            candidates: vec![GeminiCandidate {
                content: GeminiResponseContent {
                    parts: vec![GeminiResponsePart {
                        text: None,
                        function_call: Some(GeminiFunctionCall {
                            name: "get_weather".to_owned(),
                            args: serde_json::json!({"city": "Tokyo"}),
                        }),
                    }],
                },
                finish_reason: Some("STOP".to_owned()),
            }],
            usage_metadata: GeminiUsageMetadata { prompt_token_count: 10, candidates_token_count: 5 },
        };
        let (text, calls, tok_in, tok_out) = decode_response(resp);
        assert!(text.is_empty());
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].function.name, "get_weather");
        assert!(calls[0].function.arguments.contains("Tokyo"));
        assert_eq!(tok_in, 10);
        assert_eq!(tok_out, 5);
    }

    #[test]
    fn encode_message_multipart_produces_array_of_parts() {
        let msg = Message::with_image("user", "describe", "https://example.com/img.jpg");
        let c = encode_message(&msg);
        assert_eq!(c.parts.len(), 2);
    }
}
