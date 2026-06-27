use ancora_proto::ancora::{AgentSpec, EffectClass, ToolSpec};

fn get_weather(location: &str) -> String {
    format!("Weather in {location}: 22 C, partly cloudy")
}

fn calculate(expression: &str) -> f64 {
    let parts: Vec<&str> = expression.splitn(2, '+').collect();
    if parts.len() == 2 {
        let a: f64 = parts[0].trim().parse().unwrap_or(0.0);
        let b: f64 = parts[1].trim().parse().unwrap_or(0.0);
        a + b
    } else {
        0.0
    }
}

fn weather_tool_spec() -> ToolSpec {
    ToolSpec {
        name: "get_weather".to_string(),
        description: "Get weather for a location.".to_string(),
        input_schema_json: serde_json::json!({
            "type": "object",
            "properties": { "location": { "type": "string", "description": "City name" } },
            "required": ["location"]
        })
        .to_string(),
        output_schema_json: r#"{"type":"string"}"#.to_string(),
        effect_class: EffectClass::EffectRead as i32,
        idempotency_key_template: String::new(),
    }
}

#[test]
fn weather_tool_returns_result_for_city() {
    let result = get_weather("Cairo");
    assert!(result.contains("Cairo"));
    assert!(result.contains("22 C"));
}

#[test]
fn calculate_tool_adds_two_numbers() {
    let result = calculate("3 + 4");
    assert!((result - 7.0).abs() < 1e-9);
}

#[test]
fn tool_spec_has_correct_name_and_description() {
    let spec = weather_tool_spec();
    assert_eq!("get_weather", spec.name);
    assert_eq!(EffectClass::EffectRead as i32, spec.effect_class);
}

#[test]
fn tool_spec_input_schema_is_valid_json() {
    let spec = weather_tool_spec();
    let parsed: serde_json::Value = serde_json::from_str(&spec.input_schema_json).unwrap();
    assert_eq!("object", parsed["type"].as_str().unwrap());
    assert!(parsed["properties"]["location"].is_object());
}

#[test]
fn agent_spec_with_tool_is_constructible() {
    let spec = AgentSpec {
        name: "tool-agent".to_string(),
        model_id: "local-model".to_string(),
        instructions: "Use tools.".to_string(),
        output_schema_json: String::new(),
        tools: vec![weather_tool_spec()],
        max_steps: 5,
        model_retry: None,
        model_params_json: String::new(),
    };
    assert_eq!(1, spec.tools.len());
    assert_eq!("get_weather", spec.tools[0].name);
}
