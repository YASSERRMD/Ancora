# MCP Tool Use Example

Demonstrates defining `ToolSpec` objects with JSON schema, wiring them into
an `AgentSpec`, and verifying local dispatch functions.

## What it tests

- Local Rust functions (`get_weather`, `calculate`) return correct values
- `ToolSpec.name`, `description`, and `effect_class` are set correctly
- `input_schema_json` is valid JSON with the expected shape
- `AgentSpec.tools` holds the `ToolSpec` correctly

## Pattern

```rust
use ancora_proto::ancora::{AgentSpec, EffectClass, ToolSpec};

fn get_weather(location: &str) -> String {
    format!("Weather in {location}: 22 C, partly cloudy")
}

let weather_spec = ToolSpec {
    name: "get_weather".to_string(),
    description: "Get weather for a location.".to_string(),
    input_schema_json: serde_json::json!({
        "type": "object",
        "properties": { "location": { "type": "string" } },
        "required": ["location"]
    }).to_string(),
    output_schema_json: r#"{"type":"string"}"#.to_string(),
    effect_class: EffectClass::EffectRead as i32,
    idempotency_key_template: String::new(),
};

let spec = AgentSpec { tools: vec![weather_spec], .. };
assert_eq!(1, spec.tools.len());
```

## Offline

Tool function and schema tests are fully in-process.
