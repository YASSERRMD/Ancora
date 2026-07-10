use ancora_proto::ancora::{AgentSpec, ToolSpec};

/// Decode `spec_bytes` sent by a host language into a real `AgentSpec`.
///
/// Callers currently disagree on the wire format:
/// - the Python SDK sends canonical protobuf-JSON (`model_id`, `max_steps`, ...);
/// - the Go SDK sends protobuf binary;
/// - the .NET/Java/TypeScript SDKs send a JSON shape with different field
///   names (`model`, `max_tokens`, `temperature`) left over from before the
///   run engine was wired to `ancora-core`.
///
/// This never fails: unrecognized or empty input decodes to a default,
/// empty `AgentSpec` rather than rejecting the run, since `ancora_run_start`
/// has no way to report a decode error without breaking existing callers.
pub(crate) fn decode_agent_spec(bytes: &[u8]) -> AgentSpec {
    if bytes.is_empty() {
        return AgentSpec::default();
    }
    if let Ok(spec) = serde_json::from_slice::<AgentSpec>(bytes) {
        return spec;
    }
    // Any bytes that parse as JSON syntax at all are handled here, before
    // the protobuf-binary fallback below: protobuf's binary decoder is
    // permissive enough that JSON text can occasionally "succeed" as a
    // decode and silently produce garbage instead of falling through.
    if let Ok(value) = serde_json::from_slice::<serde_json::Value>(bytes) {
        return lenient_spec_from_value(&value);
    }
    if let Ok(spec) = <AgentSpec as prost::Message>::decode(bytes) {
        return spec;
    }
    AgentSpec::default()
}

/// Best-effort extraction for JSON that doesn't match the canonical
/// protobuf-JSON field names, e.g. `{"model": "...", "max_tokens": 256}`.
fn lenient_spec_from_value(value: &serde_json::Value) -> AgentSpec {
    let str_field = |keys: &[&str]| -> String {
        keys.iter()
            .find_map(|k| value.get(k).and_then(|v| v.as_str()))
            .unwrap_or_default()
            .to_owned()
    };

    let model_id = str_field(&["model_id", "model", "modelId"]);
    let instructions = str_field(&["instructions"]);
    let output_schema_json = str_field(&["output_schema_json", "outputSchemaJson"]);

    // `max_steps` bounds agent loop iterations; `max_tokens` bounds a single
    // completion's output length. They are not interchangeable, so only the
    // former is honored here.
    let max_steps = value
        .get("max_steps")
        .or_else(|| value.get("maxSteps"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    let tools = value
        .get("tools")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(lenient_tool_from_value).collect())
        .unwrap_or_default();

    let model_params_json = lenient_model_params(value);

    AgentSpec {
        name: str_field(&["name"]),
        model_id,
        instructions,
        output_schema_json,
        tools,
        max_steps,
        model_retry: None,
        model_params_json,
    }
}

fn lenient_tool_from_value(value: &serde_json::Value) -> ToolSpec {
    let name = value
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned();
    let description = value
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned();
    // Canonical wire shape carries the JSON Schema as a string
    // (`input_schema_json`); the legacy .NET/Java/TS shape carries it as a
    // nested object (`input_schema`/`inputSchema`).
    let input_schema_json = value
        .get("input_schema_json")
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .or_else(|| {
            value
                .get("input_schema")
                .or_else(|| value.get("inputSchema"))
                .map(|v| v.to_string())
        })
        .unwrap_or_default();

    ToolSpec {
        name,
        description,
        input_schema_json,
        output_schema_json: String::new(),
        effect_class: 0,
        idempotency_key_template: String::new(),
    }
}

/// Fold legacy `temperature`/`max_tokens` fields into `model_params_json` so
/// they still reach the model even when the canonical field wasn't sent.
fn lenient_model_params(value: &serde_json::Value) -> String {
    if let Some(s) = value.get("model_params_json").and_then(|v| v.as_str()) {
        return s.to_owned();
    }
    let mut params = serde_json::Map::new();
    if let Some(t) = value.get("temperature").and_then(|v| v.as_f64()) {
        params.insert("temperature".to_owned(), serde_json::json!(t));
    }
    if let Some(mt) = value.get("max_tokens").or_else(|| value.get("maxTokens")) {
        if let Some(mt) = mt.as_u64() {
            params.insert("max_tokens".to_owned(), serde_json::json!(mt));
        }
    }
    if params.is_empty() {
        String::new()
    } else {
        serde_json::Value::Object(params).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bytes_decode_to_default_spec() {
        let spec = decode_agent_spec(b"");
        assert_eq!(spec.model_id, "");
        assert_eq!(spec.max_steps, 0);
    }

    #[test]
    fn empty_json_object_decodes_to_default_spec() {
        let spec = decode_agent_spec(b"{}");
        assert_eq!(spec.model_id, "");
        assert!(spec.tools.is_empty());
    }

    #[test]
    fn canonical_snake_case_json_decodes_directly() {
        let bytes = br#"{"model_id":"gpt-4o-mini","instructions":"hi","max_steps":5}"#;
        let spec = decode_agent_spec(bytes);
        assert_eq!(spec.model_id, "gpt-4o-mini");
        assert_eq!(spec.instructions, "hi");
        assert_eq!(spec.max_steps, 5);
    }

    #[test]
    fn canonical_camel_case_json_decodes_directly() {
        let bytes = br#"{"modelId":"gpt-4o-mini","maxSteps":3}"#;
        let spec = decode_agent_spec(bytes);
        assert_eq!(spec.model_id, "gpt-4o-mini");
        assert_eq!(spec.max_steps, 3);
    }

    #[test]
    fn legacy_dotnet_shape_falls_back_to_lenient_extraction() {
        let bytes =
            br#"{"model":"llama3","instructions":"be brief","max_tokens":256,"temperature":0.7}"#;
        let spec = decode_agent_spec(bytes);
        assert_eq!(spec.model_id, "llama3");
        assert_eq!(spec.instructions, "be brief");
        let params: serde_json::Value = serde_json::from_str(&spec.model_params_json).unwrap();
        assert_eq!(params["max_tokens"], 256);
        assert_eq!(params["temperature"], 0.7);
    }

    #[test]
    fn legacy_shape_does_not_conflate_max_tokens_with_max_steps() {
        let bytes = br#"{"model":"llama3","max_tokens":4096}"#;
        let spec = decode_agent_spec(bytes);
        assert_eq!(spec.max_steps, 0, "max_tokens must not become max_steps");
    }

    #[test]
    fn legacy_shape_tool_with_object_input_schema() {
        let bytes = br#"{"model":"m","tools":[{"name":"lookup","description":"d","input_schema":{"type":"object"}}]}"#;
        let spec = decode_agent_spec(bytes);
        assert_eq!(spec.tools.len(), 1);
        assert_eq!(spec.tools[0].name, "lookup");
        assert!(spec.tools[0].input_schema_json.contains("object"));
    }

    #[test]
    fn garbage_bytes_decode_to_default_spec_without_panicking() {
        let spec = decode_agent_spec(&[0xFF, 0x00, 0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(spec.model_id, "");
    }

    #[test]
    fn protobuf_binary_round_trips() {
        let original = AgentSpec {
            name: "n".into(),
            model_id: "gpt-4o".into(),
            instructions: "sys".into(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 7,
            model_retry: None,
            model_params_json: String::new(),
        };
        let mut bytes = Vec::new();
        prost::Message::encode(&original, &mut bytes).unwrap();
        let decoded = decode_agent_spec(&bytes);
        assert_eq!(decoded.model_id, "gpt-4o");
        assert_eq!(decoded.max_steps, 7);
    }
}
