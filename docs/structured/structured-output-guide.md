# Ancora Structured Output Guide

The `ancora-structured` crate enforces type-safe agent responses via schema validation and automatic extraction.

## Defining an output schema

```rust
use ancora_structured::{OutputSchema, FieldSchema, JsonType};

let schema = OutputSchema::new("classification")
    .add_field(FieldSchema::new("category", JsonType::String, true)
        .with_description("one of: bug, feature, question"))
    .add_field(FieldSchema::new("confidence", JsonType::Number, true))
    .add_field(FieldSchema::new("tags", JsonType::Array, false));

// Pass to model as JSON schema
let json_schema = schema.to_json_schema();
```

## Validating a model response

```rust
use ancora_structured::{OutputValidator, JsonExtractor};

let text = r#"{"category": "bug", "confidence": 0.95}"#;
let value = JsonExtractor::extract(text)?;
OutputValidator::validate(&schema, &value)?;
```

## Automatic retry on validation failure

```rust
use ancora_structured::{StructuredRetry, RetryConfig};

let retry = StructuredRetry::new(RetryConfig::new(3));
let value = retry.run(&schema, |attempt, last_error| {
    // Build prompt with last_error hint if available
    call_model(prompt_with_schema_and_hint(&schema, last_error))
})?;
```

## Extracting from prose

The `JsonExtractor` handles both clean JSON and JSON embedded in prose:
- `{"key": "value"}` (pure JSON)
- `Here is the output: {"key": "value"} -- end` (prose wrapping)

It uses brace-depth matching so nested objects are handled correctly.
