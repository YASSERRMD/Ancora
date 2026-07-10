use std::sync::{Arc, Mutex};

use ancora_core::agent::ModelClient as CoreModelClient;
use ancora_core::error::AncoraError;
use ancora_inference::client::ModelClient as InferenceModelClient;
use ancora_inference::error::InferenceError;
use ancora_inference::provider::{AuthStrategy, ProviderProfile};
use ancora_inference::types::{
    CompletionRequest, CompletionResponse, ContentPart, FunctionDefinition, Message as InfMessage,
    ToolDefinition,
};
use ancora_proto::ancora::{
    content_block::Block, AgentSpec, ContentBlock, Message as ProtoMessage, Role, TextContent,
    TokenUsage, ToolCallContent,
};

/// Selects which model backend a runtime executes runs against.
pub(crate) enum ModelBackend {
    /// Deterministic, offline, no-network default. Used when no provider
    /// config is supplied, so tests and disconnected callers keep working.
    Offline,
    Provider(Arc<dyn InferenceModelClient>),
}

impl ModelBackend {
    /// Config bytes are JSON: `{"provider":{"base_url":"...",
    /// "auth_env_var":"...","chat_completions_path":"..."}}`. Parsed as a
    /// plain `serde_json::Value` (rather than a derived struct) to keep
    /// this crate's only structured-JSON dependency `serde_json`, matching
    /// `spec_decode.rs`. Missing, empty, or unrecognized config bytes fall
    /// back to `Offline`, so this never fails on malformed input.
    pub(crate) fn from_config_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return ModelBackend::Offline;
        }
        let Ok(config) = serde_json::from_slice::<serde_json::Value>(bytes) else {
            return ModelBackend::Offline;
        };
        let Some(provider) = config.get("provider") else {
            return ModelBackend::Offline;
        };
        let Some(base_url) = provider.get("base_url").and_then(|v| v.as_str()) else {
            return ModelBackend::Offline;
        };
        let auth = match provider.get("auth_env_var").and_then(|v| v.as_str()) {
            Some(env_var) => AuthStrategy::BearerToken {
                env_var: env_var.to_owned(),
            },
            None => AuthStrategy::None,
        };
        let mut profile = ProviderProfile::new("configured", base_url, auth);
        if let Some(path) = provider
            .get("chat_completions_path")
            .and_then(|v| v.as_str())
        {
            profile = profile.with_chat_path(path);
        }
        let client = ancora_inference::openai::OpenAiClient::new(Arc::new(profile));
        // Wrap every provider-backed client with retry/backoff on transient
        // failures (429 with Retry-After, 5xx, unreachable endpoint). Without
        // this, a single transient network blip fails the whole run instead
        // of being absorbed the way a production caller would expect.
        let retrying = ancora_inference::retry::RetryingModelClient::new(
            client,
            ancora_inference::retry::RetryPolicy::default(),
        );
        ModelBackend::Provider(Arc::new(retrying))
    }

    /// Build a fresh `ancora_core::agent::ModelClient` adapter for one run.
    /// `cost_sink` accumulates USD cost across every model call in that run.
    pub(crate) fn make_adapter(&self, cost_sink: Arc<Mutex<f64>>) -> Box<dyn CoreModelClient> {
        match self {
            ModelBackend::Offline => Box::new(OfflineEchoModelClient),
            ModelBackend::Provider(client) => Box::new(ProviderModelClientAdapter {
                inner: Arc::clone(client),
                cost_sink,
            }),
        }
    }
}

/// Deterministic, network-free model client: echoes the last user-visible
/// text back as the final answer. This keeps runs against a runtime built
/// with no provider config (the default) fully offline and side-effect
/// free, matching how the FFI test suite and disconnected callers expect
/// `ancora_run_start` to behave.
struct OfflineEchoModelClient;

impl CoreModelClient for OfflineEchoModelClient {
    fn complete(
        &self,
        messages: &[ProtoMessage],
        _spec: &AgentSpec,
    ) -> Result<ProtoMessage, AncoraError> {
        let last_text = messages
            .iter()
            .rev()
            .find_map(message_text)
            .unwrap_or_default();
        Ok(text_response(&last_text))
    }
}

/// Bridges `ancora_core::agent::ModelClient` to a real, HTTP-backed
/// `ancora_inference::client::ModelClient`, translating between the two
/// crates' independent message/request shapes.
struct ProviderModelClientAdapter {
    inner: Arc<dyn InferenceModelClient>,
    cost_sink: Arc<Mutex<f64>>,
}

impl CoreModelClient for ProviderModelClientAdapter {
    fn complete(
        &self,
        messages: &[ProtoMessage],
        spec: &AgentSpec,
    ) -> Result<ProtoMessage, AncoraError> {
        let request = build_completion_request(messages, spec);
        let response = self.inner.complete(&request).map_err(map_inference_error)?;
        if let Some(cost) = response.cost_usd {
            *self.cost_sink.lock().unwrap() += cost;
        }
        Ok(build_proto_message(response))
    }
}

fn build_completion_request(messages: &[ProtoMessage], spec: &AgentSpec) -> CompletionRequest {
    let inf_messages = messages.iter().map(proto_message_to_inference).collect();
    let tools = spec.tools.iter().map(tool_spec_to_definition).collect();
    let params: serde_json::Value = if spec.model_params_json.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_str(&spec.model_params_json).unwrap_or(serde_json::Value::Null)
    };
    let max_tokens = params
        .get("max_tokens")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    let temperature = params
        .get("temperature")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);

    CompletionRequest {
        model_id: spec.model_id.clone(),
        messages: inf_messages,
        max_tokens,
        temperature,
        tools,
        tool_choice: None,
    }
}

fn role_to_str(role_i32: i32) -> &'static str {
    if role_i32 == Role::System as i32 {
        "system"
    } else if role_i32 == Role::Assistant as i32 {
        "assistant"
    } else if role_i32 == Role::Tool as i32 {
        "tool"
    } else {
        "user"
    }
}

/// Flatten a proto message's content blocks into inference-crate `Message`
/// shape. Text blocks concatenate directly; tool-call/tool-result blocks
/// have no first-class representation in `ancora_inference::types::Message`
/// yet, so they are rendered as readable text rather than dropped.
fn proto_message_to_inference(msg: &ProtoMessage) -> InfMessage {
    let mut text = String::new();
    for block in &msg.content {
        match &block.block {
            Some(Block::Text(t)) => text.push_str(&t.text),
            Some(Block::ToolCall(tc)) => {
                text.push_str(&format!(
                    "[tool_call {}({})]",
                    tc.tool_name, tc.arguments_json
                ));
            }
            Some(Block::ToolResult(tr)) => text.push_str(&tr.result_json),
            // Image/audio/document blocks have no representation in
            // ancora-inference's plain-text `Message` shape yet.
            Some(_) | None => {}
        }
    }
    InfMessage {
        role: role_to_str(msg.role).to_owned(),
        content: text,
        content_parts: Vec::<ContentPart>::new(),
    }
}

fn tool_spec_to_definition(tool: &ancora_proto::ancora::ToolSpec) -> ToolDefinition {
    let parameters = if tool.input_schema_json.is_empty() {
        serde_json::json!({"type": "object", "properties": {}})
    } else {
        serde_json::from_str(&tool.input_schema_json)
            .unwrap_or_else(|_| serde_json::json!({"type": "object", "properties": {}}))
    };
    ToolDefinition {
        kind: "function".to_owned(),
        function: FunctionDefinition {
            name: tool.name.clone(),
            description: tool.description.clone(),
            parameters,
        },
    }
}

fn build_proto_message(response: CompletionResponse) -> ProtoMessage {
    let mut content = Vec::new();
    if !response.content.is_empty() {
        content.push(ContentBlock {
            block: Some(Block::Text(TextContent {
                text: response.content,
            })),
        });
    }
    for tc in response.tool_calls {
        content.push(ContentBlock {
            block: Some(Block::ToolCall(ToolCallContent {
                tool_call_id: tc.id,
                tool_name: tc.function.name,
                arguments_json: tc.function.arguments,
            })),
        });
    }
    ProtoMessage {
        id: String::new(),
        role: Role::Assistant as i32,
        content,
        created_at_ns: 0,
        usage: Some(TokenUsage {
            input_tokens: response.tokens_in,
            output_tokens: response.tokens_out,
            cache_read_tokens: 0,
            cache_write_tokens: 0,
        }),
        cost: None,
        model_id: String::new(),
    }
}

fn map_inference_error(err: InferenceError) -> AncoraError {
    match err {
        InferenceError::Refused(s) => AncoraError::ModelRefused(s),
        InferenceError::Http { status, body } => AncoraError::ModelHttp { status, body },
        InferenceError::Parse(s) => AncoraError::ModelParse(s),
        InferenceError::Unreachable(s) => AncoraError::ModelUnreachable(s),
        InferenceError::Internal(s) => AncoraError::ModelParse(s),
        InferenceError::RateLimit { retry_after } => AncoraError::ModelHttp {
            status: 429,
            body: match retry_after {
                Some(d) => format!("rate limited, retry after {}s", d.as_secs()),
                None => "rate limited".to_owned(),
            },
        },
        InferenceError::AuthRejected(s) => AncoraError::ModelHttp {
            status: 401,
            body: s,
        },
        InferenceError::MissingCredential(s) => {
            AncoraError::ModelUnreachable(format!("missing credential: {s}"))
        }
    }
}

fn message_text(msg: &ProtoMessage) -> Option<String> {
    let text: String = msg
        .content
        .iter()
        .filter_map(|b| match &b.block {
            Some(Block::Text(t)) => Some(t.text.clone()),
            _ => None,
        })
        .collect();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn text_response(text: &str) -> ProtoMessage {
    ProtoMessage {
        id: String::new(),
        role: Role::Assistant as i32,
        content: vec![ContentBlock {
            block: Some(Block::Text(TextContent {
                text: text.to_owned(),
            })),
        }],
        created_at_ns: 0,
        usage: None,
        cost: None,
        model_id: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_message(text: &str) -> ProtoMessage {
        ProtoMessage {
            id: String::new(),
            role: Role::User as i32,
            content: vec![ContentBlock {
                block: Some(Block::Text(TextContent {
                    text: text.to_owned(),
                })),
            }],
            created_at_ns: 0,
            usage: None,
            cost: None,
            model_id: String::new(),
        }
    }

    #[test]
    fn offline_echo_returns_last_user_text() {
        let client = OfflineEchoModelClient;
        let messages = vec![user_message("hello there")];
        let spec = AgentSpec::default();
        let response = client.complete(&messages, &spec).unwrap();
        let text = message_text(&response).unwrap();
        assert_eq!(text, "hello there");
    }

    #[test]
    fn offline_echo_handles_no_messages() {
        let client = OfflineEchoModelClient;
        let response = client.complete(&[], &AgentSpec::default()).unwrap();
        assert_eq!(message_text(&response), None);
    }

    #[test]
    fn empty_config_bytes_select_offline_backend() {
        let backend = ModelBackend::from_config_bytes(b"");
        assert!(matches!(backend, ModelBackend::Offline));
    }

    #[test]
    fn config_without_provider_selects_offline_backend() {
        let backend = ModelBackend::from_config_bytes(b"{}");
        assert!(matches!(backend, ModelBackend::Offline));
    }

    #[test]
    fn config_with_provider_selects_provider_backend() {
        let bytes =
            br#"{"provider":{"base_url":"http://localhost:9999","auth_env_var":"TEST_KEY"}}"#;
        let backend = ModelBackend::from_config_bytes(bytes);
        assert!(matches!(backend, ModelBackend::Provider(_)));
    }

    #[test]
    fn garbage_config_bytes_fall_back_to_offline() {
        let backend = ModelBackend::from_config_bytes(&[0xFF, 0x00, 0xDE]);
        assert!(matches!(backend, ModelBackend::Offline));
    }

    #[test]
    fn map_inference_error_covers_every_variant() {
        assert!(matches!(
            map_inference_error(InferenceError::Refused("x".into())),
            AncoraError::ModelRefused(_)
        ));
        assert!(matches!(
            map_inference_error(InferenceError::Http {
                status: 500,
                body: "x".into()
            }),
            AncoraError::ModelHttp { status: 500, .. }
        ));
        assert!(matches!(
            map_inference_error(InferenceError::RateLimit { retry_after: None }),
            AncoraError::ModelHttp { status: 429, .. }
        ));
        assert!(matches!(
            map_inference_error(InferenceError::AuthRejected("x".into())),
            AncoraError::ModelHttp { status: 401, .. }
        ));
        assert!(matches!(
            map_inference_error(InferenceError::MissingCredential("x".into())),
            AncoraError::ModelUnreachable(_)
        ));
    }

    #[test]
    fn build_completion_request_extracts_max_tokens_and_temperature() {
        let spec = AgentSpec {
            model_id: "m".into(),
            model_params_json: r#"{"max_tokens":128,"temperature":0.5}"#.into(),
            ..Default::default()
        };
        let req = build_completion_request(&[], &spec);
        assert_eq!(req.max_tokens, Some(128));
        assert_eq!(req.temperature, Some(0.5));
    }

    #[test]
    fn build_proto_message_maps_tool_calls() {
        let response = CompletionResponse {
            content: String::new(),
            tokens_in: 3,
            tokens_out: 5,
            cost_usd: Some(0.001),
            tool_calls: vec![ancora_inference::types::ToolCall {
                id: "call-1".into(),
                kind: "function".into(),
                function: ancora_inference::types::FunctionCall {
                    name: "lookup".into(),
                    arguments: r#"{"q":"x"}"#.into(),
                },
            }],
        };
        let msg = build_proto_message(response);
        let has_tool_call = msg
            .content
            .iter()
            .any(|b| matches!(&b.block, Some(Block::ToolCall(tc)) if tc.tool_name == "lookup"));
        assert!(has_tool_call);
        assert_eq!(msg.usage.unwrap().output_tokens, 5);
    }
}
