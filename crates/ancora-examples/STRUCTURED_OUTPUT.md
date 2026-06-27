# Structured Output Example

Demonstrates deriving a JSON Schema from Rust structs with `serde` attributes
and wiring it into an `AgentSpec` as `output_schema_json`.

## What it tests

- A Rust `struct` with `#[derive(Serialize, Deserialize)]` serializes with
  the correct field names
- `serde_json::to_string` / `from_str` round-trips cleanly
- Schema JSON is a valid JSON object with `type: "object"` and `properties`
- `AgentSpec.output_schema_json` holds the schema

## Pattern

```rust
#[derive(Serialize, Deserialize, PartialEq)]
struct AnalysisResult {
    summary: String,
    sentiment: String,
    score: f64,
}

let schema = serde_json::json!({
    "type": "object",
    "properties": {
        "summary":   { "type": "string" },
        "sentiment": { "type": "string" },
        "score":     { "type": "number" }
    },
    "required": ["summary", "sentiment", "score"]
}).to_string();

let spec = AgentSpec { output_schema_json: schema, .. };
```

## Offline

All tests run entirely in-process with `serde_json`.
