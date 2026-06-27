use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::client::ModelClient;
use crate::error::InferenceError;
use crate::provider::ProviderProfile;
use crate::types::{CompletionRequest, CompletionResponse, FunctionCall, FunctionDefinition, Message, ToolCall, ToolDefinition};

// ---- Wire types: chat history ----------------------------------------------

/// A single turn in the Cohere chat history.
///
/// Cohere uses "USER" and "CHATBOT" (uppercase) unlike OpenAI's "user"/"assistant".
#[derive(Debug, Serialize, Clone)]
pub(crate) struct CohereChatTurn {
    pub role: String,
    pub message: String,
}

/// Convert a message role to Cohere's uppercase role label.
///
/// Cohere: USER for all human turns, CHATBOT for all assistant turns.
pub(crate) fn cohere_role(role: &str) -> &'static str {
    match role {
        "assistant" | "CHATBOT" | "chatbot" => "CHATBOT",
        _ => "USER",
    }
}

/// Build the `chat_history` array from all non-system, non-last messages.
///
/// Cohere's API separates the current user message (goes as `message`)
/// from the prior conversation (goes as `chat_history`). System messages
/// are extracted separately as `preamble`.
pub(crate) fn build_chat_history(messages: &[&Message]) -> Vec<CohereChatTurn> {
    // All messages except the last one go into chat_history.
    messages.iter().take(messages.len().saturating_sub(1)).map(|m| CohereChatTurn {
        role: cohere_role(&m.role).to_owned(),
        message: m.content.clone(),
    }).collect()
}

/// Extract the current message (last non-system turn) and chat history.
///
/// Returns `(current_message, chat_history)`. If there are no non-system
/// messages, current_message is an empty string.
pub(crate) fn split_messages(messages: &[Message]) -> (String, Vec<CohereChatTurn>) {
    let non_system: Vec<&Message> = messages.iter().filter(|m| m.role != "system").collect();
    let current = non_system.last().map(|m| m.content.clone()).unwrap_or_default();
    let history = build_chat_history(&non_system);
    (current, history)
}

// ---- Wire types: tools -----------------------------------------------------

/// A single parameter definition in Cohere's tool format.
#[derive(Debug, Serialize, Clone)]
pub(crate) struct CohereParamDef {
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// A Cohere tool definition.
#[derive(Debug, Serialize, Clone)]
pub(crate) struct CohereToolDef {
    pub name: String,
    pub description: String,
    pub parameter_definitions: HashMap<String, CohereParamDef>,
}

/// Convert a `ToolDefinition` into Cohere's `parameter_definitions` format.
///
/// Cohere does not use JSON Schema objects; instead it uses a flat map from
/// parameter name to `{description, type, required}`.
pub(crate) fn encode_tool(t: &ToolDefinition) -> CohereToolDef {
    encode_function_as_cohere_tool(&t.function)
}

pub(crate) fn encode_function_as_cohere_tool(f: &FunctionDefinition) -> CohereToolDef {
    let mut param_defs = HashMap::new();
    if let Some(props) = f.parameters.get("properties").and_then(|v| v.as_object()) {
        let required_list: Vec<&str> = f.parameters
            .get("required")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        for (name, schema) in props {
            let description = schema.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let kind = schema.get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("str")
                .to_owned();
            let required = Some(required_list.contains(&name.as_str()));
            param_defs.insert(name.clone(), CohereParamDef { description, kind, required });
        }
    }

    CohereToolDef {
        name: f.name.clone(),
        description: f.description.clone(),
        parameter_definitions: param_defs,
    }
}

// ---- Wire types: response --------------------------------------------------

/// Cohere response tool call entry.
#[derive(Debug, Deserialize)]
pub(crate) struct CohereToolCall {
    pub name: String,
    pub parameters: serde_json::Value,
}

/// Cohere token usage sub-object.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct CohereTokens {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

/// Cohere response meta object.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct CohereMeta {
    #[serde(default)]
    pub tokens: CohereTokens,
}

/// Cohere non-streaming response body.
#[derive(Debug, Deserialize)]
pub(crate) struct CohereResponse {
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub tool_calls: Vec<CohereToolCall>,
    #[serde(default)]
    pub meta: CohereMeta,
}

/// Parse a Cohere JSON response body into a `CompletionResponse`.
pub(crate) fn parse_response(
    body: &str,
    _model_id: &str,
    cost_usd: Option<f64>,
) -> Result<CompletionResponse, InferenceError> {
    let resp: CohereResponse = serde_json::from_str(body)
        .map_err(|e| InferenceError::Parse(e.to_string()))?;

    let tool_calls: Vec<ToolCall> = resp.tool_calls.into_iter().map(|tc| {
        ToolCall {
            id: String::new(),
            kind: "function".to_owned(),
            function: FunctionCall {
                name: tc.name,
                arguments: tc.parameters.to_string(),
            },
        }
    }).collect();

    Ok(CompletionResponse {
        content: resp.text,
        tokens_in: resp.meta.tokens.input_tokens,
        tokens_out: resp.meta.tokens.output_tokens,
        cost_usd,
        tool_calls,
    })
}

// ---- Streaming -------------------------------------------------------------

/// Cohere streaming event types (the `event_type` field on each chunk).
/// Cohere uses `\ndata: {...}` SSE lines where the JSON has an `event_type`.
#[derive(Debug, Deserialize)]
#[serde(tag = "event_type", rename_all = "kebab-case")]
pub(crate) enum CohereStreamEvent {
    #[serde(rename = "text-generation")]
    TextGeneration { text: String },
    #[serde(rename = "stream-end")]
    StreamEnd {
        #[serde(default)]
        finish_reason: String,
    },
    #[serde(other)]
    Other,
}

/// Parse a single SSE line from a Cohere streaming response.
///
/// Cohere streams `data: {...}` lines. Each JSON object has an `event_type`
/// field: "text-generation" carries a `text` chunk; "stream-end" signals
/// the end of the stream.
pub fn parse_sse_line(line: &str) -> Option<crate::types::TokenEvent> {
    let data = line.strip_prefix("data: ")?;
    let evt: CohereStreamEvent = serde_json::from_str(data).ok()?;
    match evt {
        CohereStreamEvent::TextGeneration { text } => {
            Some(crate::types::TokenEvent { text, finished: false })
        }
        CohereStreamEvent::StreamEnd { .. } => {
            Some(crate::types::TokenEvent { text: String::new(), finished: true })
        }
        CohereStreamEvent::Other => None,
    }
}

/// Collect system messages into a single preamble string.
///
/// Cohere uses a top-level `preamble` field instead of a system role in the
/// messages array. Multiple system messages are joined with a blank line.
pub(crate) fn extract_preamble(messages: &[Message]) -> Option<String> {
    let parts: Vec<&str> = messages.iter()
        .filter(|m| m.role == "system")
        .map(|m| m.content.as_str())
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n\n"))
    }
}

// ---- CohereClient ----------------------------------------------------------

/// Cohere request body wire type.
#[derive(Debug, Serialize)]
struct WireChatRequest {
    model: String,
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    chat_history: Vec<CohereChatTurn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preamble: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<CohereToolDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// HTTP adapter for the Cohere Chat API.
///
/// Cohere's wire format differs from OpenAI: conversations are split into
/// `message` (current turn) + `chat_history` (prior turns), and system
/// instructions go into `preamble`.
pub struct CohereClient {
    profile: Arc<ProviderProfile>,
}

impl CohereClient {
    pub fn new(profile: Arc<ProviderProfile>) -> Self {
        Self { profile }
    }

    pub(crate) fn build_request_body(
        &self,
        request: &CompletionRequest,
        stream: bool,
    ) -> Result<serde_json::Value, InferenceError> {
        let model_id = self.profile.resolve_model_id(&request.model_id).to_owned();
        let preamble = extract_preamble(&request.messages);
        let (message, chat_history) = split_messages(&request.messages);
        let tools: Vec<CohereToolDef> = request.tools.iter().map(encode_tool).collect();
        let wire = WireChatRequest {
            model: model_id,
            message,
            chat_history,
            preamble,
            tools,
            stream: if stream { Some(true) } else { None },
        };
        let mut body = serde_json::to_value(&wire)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        self.profile.request_transforms.apply(&mut body);
        Ok(body)
    }

    pub fn parse_response(
        &self,
        body: &str,
        model_id: &str,
    ) -> Result<CompletionResponse, InferenceError> {
        let resp: CohereResponse = serde_json::from_str(body)
            .map_err(|e| InferenceError::Parse(e.to_string()))?;
        let tokens_in = resp.meta.tokens.input_tokens;
        let tokens_out = resp.meta.tokens.output_tokens;
        let cost_usd = self.profile
            .model_meta(model_id)
            .and_then(|m| m.compute_cost(tokens_in, tokens_out, 0));
        let tool_calls: Vec<ToolCall> = resp.tool_calls.into_iter().map(|tc| ToolCall {
            id: String::new(),
            kind: "function".to_owned(),
            function: FunctionCall { name: tc.name, arguments: tc.parameters.to_string() },
        }).collect();
        Ok(CompletionResponse {
            content: resp.text,
            tokens_in,
            tokens_out,
            cost_usd,
            tool_calls,
        })
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
        on_token: &mut dyn FnMut(crate::types::TokenEvent),
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
}

impl ModelClient for CohereClient {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, InferenceError> {
        let body = self.build_request_body(request, false)?;
        let resp_str = self.post(&body)?;
        self.parse_response(&resp_str, &request.model_id)
    }

    fn stream_complete(
        &self,
        request: &CompletionRequest,
        on_token: &mut dyn FnMut(crate::types::TokenEvent),
    ) -> Result<CompletionResponse, InferenceError> {
        let body = self.build_request_body(request, true)?;
        let mut content = String::new();
        self.post_stream(&body, &mut |event: crate::types::TokenEvent| {
            if !event.text.is_empty() {
                content.push_str(&event.text);
            }
            on_token(event);
        })?;
        let model_id = self.profile.resolve_model_id(&request.model_id).to_owned();
        let (tokens_in, tokens_out) = (0u64, 0u64);
        let cost_usd = self.profile
            .model_meta(&model_id)
            .and_then(|m| m.compute_cost(tokens_in, tokens_out, 0));
        Ok(CompletionResponse { content, tokens_in, tokens_out, cost_usd, tool_calls: vec![] })
    }
}

#[cfg(test)]
const FIXTURE: &str = r#"{"text":"Hello from Cohere!","generation_id":"gen-01","tool_calls":[],"meta":{"tokens":{"input_tokens":12,"output_tokens":5}}}"#;

#[cfg(test)]
const FIXTURE_TOOL: &str = r#"{"text":"","generation_id":"gen-02","tool_calls":[{"name":"get_weather","parameters":{"location":"San Francisco"}}],"meta":{"tokens":{"input_tokens":20,"output_tokens":8}}}"#;

#[cfg(test)]
const FIXTURE_STREAM: &[&str] = &[
    r#"data: {"event_type":"text-generation","text":"Hello"}"#,
    r#"data: {"event_type":"text-generation","text":" from"}"#,
    r#"data: {"event_type":"text-generation","text":" Cohere"}"#,
    r#"data: {"event_type":"stream-end","finish_reason":"COMPLETE","response":{}}"#,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FunctionDefinition, Message, ToolDefinition};

    #[test]
    fn cohere_role_user_maps_to_user() {
        assert_eq!(cohere_role("user"), "USER");
    }

    #[test]
    fn cohere_role_assistant_maps_to_chatbot() {
        assert_eq!(cohere_role("assistant"), "CHATBOT");
    }

    #[test]
    fn split_messages_current_is_last_user_message() {
        let msgs = vec![
            Message::text("user", "Hello"),
            Message::text("assistant", "Hi there"),
            Message::text("user", "What is the weather?"),
        ];
        let (current, history) = split_messages(&msgs);
        assert_eq!(current, "What is the weather?");
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn split_messages_history_has_correct_roles() {
        let msgs = vec![
            Message::text("user", "Hello"),
            Message::text("assistant", "Hi"),
            Message::text("user", "Bye"),
        ];
        let (_, history) = split_messages(&msgs);
        assert_eq!(history[0].role, "USER");
        assert_eq!(history[1].role, "CHATBOT");
    }

    #[test]
    fn split_messages_single_message_empty_history() {
        let msgs = vec![Message::text("user", "First")];
        let (current, history) = split_messages(&msgs);
        assert_eq!(current, "First");
        assert!(history.is_empty());
    }

    #[test]
    fn split_messages_system_excluded_from_chat_history() {
        let msgs = vec![
            Message::text("system", "Be helpful"),
            Message::text("user", "Hi"),
        ];
        let (current, history) = split_messages(&msgs);
        assert_eq!(current, "Hi");
        assert!(history.is_empty());
    }

    #[test]
    fn extract_preamble_single_system_message() {
        let msgs = vec![
            Message::text("system", "You are a helpful assistant."),
            Message::text("user", "Hi"),
        ];
        let preamble = extract_preamble(&msgs);
        assert_eq!(preamble, Some("You are a helpful assistant.".to_owned()));
    }

    #[test]
    fn extract_preamble_multiple_system_messages_joined() {
        let msgs = vec![
            Message::text("system", "Part one."),
            Message::text("system", "Part two."),
            Message::text("user", "Hello"),
        ];
        let preamble = extract_preamble(&msgs);
        assert_eq!(preamble, Some("Part one.\n\nPart two.".to_owned()));
    }

    #[test]
    fn extract_preamble_no_system_returns_none() {
        let msgs = vec![Message::text("user", "Hello")];
        assert_eq!(extract_preamble(&msgs), None);
    }

    fn make_tool(name: &str, desc: &str, params: serde_json::Value) -> ToolDefinition {
        ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: name.to_owned(),
                description: desc.to_owned(),
                parameters: params,
            },
        }
    }

    #[test]
    fn encode_tool_has_correct_name_and_description() {
        let tool = make_tool("get_weather", "Get current weather", serde_json::json!({
            "type": "object",
            "properties": {
                "location": {"type": "str", "description": "City name"}
            },
            "required": ["location"]
        }));
        let def = encode_tool(&tool);
        assert_eq!(def.name, "get_weather");
        assert_eq!(def.description, "Get current weather");
    }

    #[test]
    fn encode_tool_parameter_definitions_has_location() {
        let tool = make_tool("get_weather", "Get current weather", serde_json::json!({
            "type": "object",
            "properties": {
                "location": {"type": "str", "description": "City name"}
            },
            "required": ["location"]
        }));
        let def = encode_tool(&tool);
        let param = def.parameter_definitions.get("location").unwrap();
        assert_eq!(param.kind, "str");
        assert_eq!(param.description, "City name");
        assert_eq!(param.required, Some(true));
    }

    #[test]
    fn encode_tool_optional_parameter_marked_false() {
        let tool = make_tool("search", "Search", serde_json::json!({
            "type": "object",
            "properties": {
                "query": {"type": "str", "description": "Search query"},
                "limit": {"type": "int", "description": "Max results"}
            },
            "required": ["query"]
        }));
        let def = encode_tool(&tool);
        let limit = def.parameter_definitions.get("limit").unwrap();
        assert_eq!(limit.required, Some(false));
    }

    fn client() -> CohereClient {
        use crate::providers::cohere::build_cohere_profile;
        use std::sync::Arc;
        CohereClient::new(Arc::new(build_cohere_profile()))
    }

    #[test]
    fn cohere_recorded_fixture_completes() {
        let resp = client().parse_response(FIXTURE, "command-r-plus").unwrap();
        assert_eq!(resp.content, "Hello from Cohere!");
        assert_eq!(resp.tokens_in, 12);
        assert_eq!(resp.tokens_out, 5);
    }

    #[test]
    fn cohere_fixture_content_non_empty() {
        let resp = client().parse_response(FIXTURE, "command-r-plus").unwrap();
        assert!(!resp.content.is_empty());
    }

    #[test]
    fn cohere_fixture_no_tool_calls() {
        let resp = client().parse_response(FIXTURE, "command-r-plus").unwrap();
        assert!(resp.tool_calls.is_empty());
    }

    #[test]
    fn cohere_tool_call_fixture_parsed() {
        let resp = client().parse_response(FIXTURE_TOOL, "command-r-plus").unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        assert_eq!(resp.tool_calls[0].function.name, "get_weather");
    }

    #[test]
    fn cohere_tool_call_arguments_contain_location() {
        let resp = client().parse_response(FIXTURE_TOOL, "command-r-plus").unwrap();
        let args: serde_json::Value =
            serde_json::from_str(&resp.tool_calls[0].function.arguments).unwrap();
        assert_eq!(args["location"], "San Francisco");
    }

    #[test]
    fn cohere_tool_call_tokens_correct() {
        let resp = client().parse_response(FIXTURE_TOOL, "command-r-plus").unwrap();
        assert_eq!(resp.tokens_in, 20);
        assert_eq!(resp.tokens_out, 8);
    }
}
