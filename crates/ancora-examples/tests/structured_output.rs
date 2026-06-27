use ancora_proto::ancora::{AgentSpec, EffectClass, ToolSpec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AnalysisResult {
    summary: String,
    sentiment: String,
    score: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ClassificationResult {
    label: String,
    confidence: f64,
    categories: Vec<String>,
}

fn analysis_schema_json() -> String {
    serde_json::json!({
        "type": "object",
        "properties": {
            "summary":   { "type": "string", "description": "Brief summary" },
            "sentiment": { "type": "string", "description": "positive, neutral, or negative" },
            "score":     { "type": "number", "description": "Confidence 0-1" }
        },
        "required": ["summary", "sentiment", "score"]
    })
    .to_string()
}

#[test]
fn analysis_result_serializes_with_correct_keys() {
    let result = AnalysisResult {
        summary: "All good".to_string(),
        sentiment: "positive".to_string(),
        score: 0.9,
    };
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("\"summary\""));
    assert!(json.contains("\"sentiment\""));
    assert!(json.contains("\"score\""));
}

#[test]
fn analysis_result_round_trips_through_json() {
    let original = AnalysisResult {
        summary: "Test".to_string(),
        sentiment: "neutral".to_string(),
        score: 0.5,
    };
    let json = serde_json::to_string(&original).unwrap();
    let decoded: AnalysisResult = serde_json::from_str(&json).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn schema_json_is_valid_json_object() {
    let schema = analysis_schema_json();
    let parsed: serde_json::Value = serde_json::from_str(&schema).unwrap();
    assert_eq!("object", parsed["type"].as_str().unwrap());
    assert!(parsed["properties"].is_object());
}

#[test]
fn schema_json_contains_all_required_fields() {
    let schema = analysis_schema_json();
    let parsed: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let required = parsed["required"].as_array().unwrap();
    let keys: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
    assert!(keys.contains(&"summary"));
    assert!(keys.contains(&"sentiment"));
    assert!(keys.contains(&"score"));
}

#[test]
fn agent_spec_with_output_schema_is_constructible() {
    let spec = AgentSpec {
        name: "analyzer".to_string(),
        model_id: "local-model".to_string(),
        instructions: "Analyze the text.".to_string(),
        output_schema_json: analysis_schema_json(),
        tools: vec![],
        max_steps: 5,
        model_retry: None,
        model_params_json: String::new(),
    };
    assert!(!spec.output_schema_json.is_empty());
    assert_eq!("analyzer", spec.name);
}

#[test]
fn classification_result_round_trips() {
    let original = ClassificationResult {
        label: "finance".to_string(),
        confidence: 0.85,
        categories: vec!["finance".to_string(), "economics".to_string()],
    };
    let json = serde_json::to_string(&original).unwrap();
    let decoded: ClassificationResult = serde_json::from_str(&json).unwrap();
    assert_eq!(original, decoded);
}
