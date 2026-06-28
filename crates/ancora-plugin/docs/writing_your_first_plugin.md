# Writing Your First Plugin

This guide walks through creating a minimal tool plugin for the Ancora agent framework.

## Step 1 - Add the dependency

In your plugin crate's `Cargo.toml`:

```toml
[dependencies]
ancora-plugin = { path = "../ancora-plugin" }
```

## Step 2 - Implement the trait

```rust
use ancora_plugin::tool_ext::{ArgSchema, ArgType, ToolError, ToolInput, ToolOutput, ToolPlugin, ToolSpec, Value};

pub struct UpperCaseTool {
    spec: ToolSpec,
}

impl UpperCaseTool {
    pub fn new() -> Self {
        Self {
            spec: ToolSpec {
                name: "uppercase".to_string(),
                description: "Convert text to uppercase.".to_string(),
                args: vec![ArgSchema {
                    name: "text".to_string(),
                    description: "Text to convert.".to_string(),
                    required: true,
                    arg_type: ArgType::String,
                }],
            },
        }
    }
}

impl ToolPlugin for UpperCaseTool {
    fn spec(&self) -> &ToolSpec { &self.spec }

    fn call(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let text = input.args.get("text")
            .ok_or_else(|| ToolError::MissingArg("text".into()))?;
        let s = text.as_str()
            .ok_or_else(|| ToolError::InvalidArg { name: "text".into(), reason: "expected string".into() })?;
        Ok(ToolOutput {
            value: Value::Str(s.to_uppercase()),
            summary: None,
        })
    }
}
```

## Step 3 - Declare a manifest

```rust
use ancora_plugin::manifest::{ManifestBuilder, PluginKind, SemVer};

let manifest = ManifestBuilder::new()
    .id("uppercase-tool")
    .name("Uppercase Tool")
    .version(SemVer::new(1, 0, 0))
    .sdk_range(SemVer::new(1, 0, 0), SemVer::new(1, 99, 0))
    .kind(PluginKind::Tool)
    .scope("tool:execute")
    .build()
    .expect("valid manifest");
```

## Step 4 - Register the plugin

```rust
use ancora_plugin::discovery::PluginRegistry;
use ancora_plugin::compatibility::check_current;

check_current(&manifest).expect("SDK compatible");

let mut registry = PluginRegistry::new();
registry.register(manifest).expect("no duplicate id");
```

## Step 5 - Test

Write unit tests that call your plugin's `call()` method directly. No network access required. Use the built-in `EchoTool` as a reference for test structure.
