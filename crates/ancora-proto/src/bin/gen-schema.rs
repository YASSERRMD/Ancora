//! Generates JSON Schema files for each major Ancora proto type.
//!
//! Run with: cargo run -p ancora-proto --bin gen-schema -- <output-dir>
//! Default output dir: spec/schema (relative to the workspace root).

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("spec/schema"));

    fs::create_dir_all(&out_dir).expect("create output directory");

    let schemas = build_schemas();
    for (name, schema) in &schemas {
        let path = out_dir.join(format!("{name}.json"));
        let json = serde_json::to_string_pretty(schema).expect("serialize schema");
        fs::write(&path, json + "\n").unwrap_or_else(|_| panic!("write {name}.json"));
        eprintln!("wrote {}", path.display());
    }

    eprintln!("generated {} schemas", schemas.len());
}

fn build_schemas() -> BTreeMap<String, serde_json::Value> {
    let mut m = BTreeMap::new();

    m.insert("Message".to_string(), message_schema());
    m.insert("ContentBlock".to_string(), content_block_schema());
    m.insert("AgentSpec".to_string(), agent_spec_schema());
    m.insert("ToolSpec".to_string(), tool_spec_schema());
    m.insert("JournalEvent".to_string(), journal_event_schema());
    m.insert("TokenUsage".to_string(), token_usage_schema());
    m.insert("Cost".to_string(), cost_schema());

    m
}

fn message_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://ancora.dev/schema/Message.json",
        "title": "Message",
        "description": "A single turn in a conversation",
        "type": "object",
        "properties": {
            "id":           { "type": "string" },
            "role":         { "$ref": "#/$defs/Role" },
            "content":      { "type": "array", "items": { "$ref": "#/$defs/ContentBlock" } },
            "createdAtNs":  { "type": "integer", "description": "Unix epoch, nanoseconds" },
            "usage":        { "$ref": "#/$defs/TokenUsage" },
            "cost":         { "$ref": "#/$defs/Cost" },
            "modelId":      { "type": "string" }
        },
        "required": ["id", "role"],
        "$defs": {
            "Role": {
                "type": "string",
                "enum": ["ROLE_UNSPECIFIED", "ROLE_SYSTEM", "ROLE_USER", "ROLE_ASSISTANT", "ROLE_TOOL"]
            },
            "ContentBlock": { "$ref": "ContentBlock.json" },
            "TokenUsage":   { "$ref": "TokenUsage.json" },
            "Cost":         { "$ref": "Cost.json" }
        }
    })
}

fn content_block_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://ancora.dev/schema/ContentBlock.json",
        "title": "ContentBlock",
        "description": "One content block within a message (exactly one variant present)",
        "type": "object",
        "oneOf": [
            {
                "properties": {
                    "text": {
                        "type": "object",
                        "properties": { "text": { "type": "string" } },
                        "required": ["text"]
                    }
                },
                "required": ["text"]
            },
            {
                "properties": {
                    "toolCall": {
                        "type": "object",
                        "properties": {
                            "toolCallId": { "type": "string" },
                            "toolName":   { "type": "string" },
                            "argumentsJson": { "type": "string" }
                        },
                        "required": ["toolCallId", "toolName", "argumentsJson"]
                    }
                },
                "required": ["toolCall"]
            },
            {
                "properties": {
                    "toolResult": {
                        "type": "object",
                        "properties": {
                            "toolCallId": { "type": "string" },
                            "resultJson": { "type": "string" },
                            "isError":    { "type": "boolean" }
                        },
                        "required": ["toolCallId", "resultJson"]
                    }
                },
                "required": ["toolResult"]
            },
            {
                "properties": {
                    "image": {
                        "type": "object",
                        "properties": {
                            "inlineBase64": { "type": "string" },
                            "url":          { "type": "string" },
                            "mediaType":    { "type": "string" }
                        }
                    }
                },
                "required": ["image"]
            },
            {
                "properties": {
                    "audio": {
                        "type": "object",
                        "properties": {
                            "inlineBase64": { "type": "string" },
                            "url":          { "type": "string" },
                            "mediaType":    { "type": "string" }
                        }
                    }
                },
                "required": ["audio"]
            },
            {
                "properties": {
                    "document": {
                        "type": "object",
                        "properties": {
                            "inlineBase64": { "type": "string" },
                            "url":          { "type": "string" },
                            "mediaType":    { "type": "string" },
                            "filename":     { "type": "string" }
                        }
                    }
                },
                "required": ["document"]
            }
        ]
    })
}

fn agent_spec_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://ancora.dev/schema/AgentSpec.json",
        "title": "AgentSpec",
        "description": "Specifies a single agent in the graph",
        "type": "object",
        "properties": {
            "name":             { "type": "string" },
            "modelId":          { "type": "string" },
            "instructions":     { "type": "string" },
            "outputSchemaJson": { "type": "string" },
            "tools":            { "type": "array", "items": { "$ref": "ToolSpec.json" } },
            "maxSteps":         { "type": "integer", "minimum": 0 },
            "modelRetry":       { "$ref": "#/$defs/RetryPolicy" },
            "modelParamsJson":  { "type": "string" }
        },
        "required": ["name", "modelId"],
        "$defs": {
            "RetryPolicy": {
                "type": "object",
                "properties": {
                    "maxAttempts":       { "type": "integer", "minimum": 0 },
                    "initialBackoffMs":  { "type": "integer", "minimum": 0 },
                    "maxBackoffMs":      { "type": "integer", "minimum": 0 },
                    "jitter":            { "type": "number", "minimum": 0.0, "maximum": 1.0 }
                }
            }
        }
    })
}

fn tool_spec_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://ancora.dev/schema/ToolSpec.json",
        "title": "ToolSpec",
        "description": "Describes a single tool that an agent may invoke",
        "type": "object",
        "properties": {
            "name":                    { "type": "string" },
            "description":             { "type": "string" },
            "inputSchemaJson":         { "type": "string" },
            "outputSchemaJson":        { "type": "string" },
            "effectClass":             { "$ref": "#/$defs/EffectClass" },
            "idempotencyKeyTemplate":  { "type": "string" }
        },
        "required": ["name", "effectClass"],
        "$defs": {
            "EffectClass": {
                "type": "string",
                "enum": ["EFFECT_UNSPECIFIED", "EFFECT_PURE", "EFFECT_READ", "EFFECT_WRITE"]
            }
        }
    })
}

fn journal_event_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://ancora.dev/schema/JournalEvent.json",
        "title": "JournalEvent",
        "description": "A single entry in the durable event journal",
        "type": "object",
        "properties": {
            "eventId":      { "type": "string" },
            "runId":        { "type": "string" },
            "seq":          { "type": "integer", "minimum": 0 },
            "recordedAtNs": { "type": "integer" },
            "runStarted":              { "type": "object" },
            "nodeEntered":             { "type": "object" },
            "nodeExited":              { "type": "object" },
            "activityRecorded":        { "type": "object" },
            "humanDecisionRequested":  { "type": "object" },
            "humanDecisionReceived":   { "type": "object" },
            "runCompleted":            { "type": "object" },
            "error":                   { "type": "object" },
            "retryScheduled":          { "type": "object" },
            "runCancelled":            { "type": "object" }
        },
        "required": ["eventId", "runId", "seq", "recordedAtNs"]
    })
}

fn token_usage_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://ancora.dev/schema/TokenUsage.json",
        "title": "TokenUsage",
        "type": "object",
        "properties": {
            "inputTokens":       { "type": "integer", "minimum": 0 },
            "outputTokens":      { "type": "integer", "minimum": 0 },
            "cacheReadTokens":   { "type": "integer", "minimum": 0 },
            "cacheWriteTokens":  { "type": "integer", "minimum": 0 }
        }
    })
}

fn cost_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://ancora.dev/schema/Cost.json",
        "title": "Cost",
        "description": "Cost amounts in micro-cents (millionths of a US cent)",
        "type": "object",
        "properties": {
            "inputMicroCents":       { "type": "integer", "minimum": 0 },
            "outputMicroCents":      { "type": "integer", "minimum": 0 },
            "cacheReadMicroCents":   { "type": "integer", "minimum": 0 },
            "cacheWriteMicroCents":  { "type": "integer", "minimum": 0 }
        }
    })
}
