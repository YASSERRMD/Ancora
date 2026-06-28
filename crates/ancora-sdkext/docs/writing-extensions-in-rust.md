# Writing Extensions in Rust

This guide covers how to author an Ancora tool extension in Rust using the
`ancora-sdkext` crate.

## Quick start

Add `ancora-sdkext` to your `Cargo.toml` dependencies, then implement the
`ToolExtension` trait.

```rust
use ancora_sdkext::rs_traits::{ExtensionError, ToolExtension, ToolMeta, Value};
use std::collections::HashMap;

pub struct MyTool;

impl ToolExtension for MyTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("my_tool", "Does something useful.", "1.0.0")
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        let input = args
            .get("input")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("'input' required".to_string()))?;
        Ok(Value::string(input.to_uppercase()))
    }
}
```

## The `ToolMeta` struct

Every extension must return a `ToolMeta` with three non-empty fields:

| Field | Description |
|-------|-------------|
| `name` | Unique identifier used when dispatching calls |
| `description` | Human-readable description |
| `version` | SemVer string |

## The `Value` enum

Arguments and return values use the `Value` enum, which covers the JSON value
space:

- `Value::Null`
- `Value::Bool(bool)`
- `Value::Int(i64)`
- `Value::Float(f64)`
- `Value::Str(String)`
- `Value::Array(Vec<Value>)`
- `Value::Map(HashMap<String, Value>)`

## Error handling

Return `Err(ExtensionError::InvalidArgument(...))` for bad inputs.
Return `Err(ExtensionError::ExecutionFailed(...))` for runtime problems.
Never panic in library code.

## Health checks

Override `health_check` to let the runtime verify your extension is operational:

```rust
fn health_check(&self) -> Result<(), ExtensionError> {
    // verify DB connection, config, etc.
    Ok(())
}
```

## Registration

```rust
use ancora_sdkext::registration::{ExtensionRegistry, register_rust_extension};
use std::sync::Arc;

let registry = ExtensionRegistry::new();
register_rust_extension(&registry, Arc::new(MyTool)).unwrap();
```

## Interop kit

Run the parity checks before shipping:

```rust
use ancora_sdkext::parity::InteropKit;

let results = InteropKit::run_all(&MyTool);
assert!(results.iter().all(|r| r.passed));
```
