# Authoring a CLI Plugin

This document walks through the process of creating a new CLI plugin for Ancora.

## 1. Implement the CliPlugin Trait

```rust
use ancora_cliplugin::interface::{
    CliPlugin, CommandSpec, ExecContext, ExecOutput, PluginMeta, PluginResult,
};

pub struct MyPlugin {
    meta: PluginMeta,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "acme.myplugin",          // unique id
                "My Plugin",             // display name
                "1.0.0",                 // semver version
                "Does useful things",    // description
                "Acme Corp",             // author
            ),
        }
    }
}

impl CliPlugin for MyPlugin {
    fn meta(&self) -> &PluginMeta { &self.meta }

    fn commands(&self) -> Vec<CommandSpec> {
        vec![
            CommandSpec::new("my-cmd", "Does a thing", "Detailed help for my-cmd"),
        ]
    }

    fn execute(&self, command: &str, ctx: ExecContext) -> PluginResult<ExecOutput> {
        match command {
            "my-cmd" => Ok(ExecOutput::success(vec!["done".into()])),
            other => Err(ancora_cliplugin::interface::PluginError::ExecutionFailed(
                format!("unknown command: {}", other),
            )),
        }
    }
}
```

## 2. Create a plugin.toml Manifest

Place a `plugin.toml` file at the root of your plugin directory:

```toml
id = "acme.myplugin"
name = "My Plugin"
version = "1.0.0"
```

## 3. Declare Required Permissions

Document the permission scopes your plugin needs in its README. At runtime,
the host CLI grants these scopes before your plugin is allowed to execute
sensitive operations. Supported scopes:

- `fs:read` - read from the file system
- `fs:write` - write to the file system
- `network` - outbound network requests
- `exec` - spawn sub-processes
- `env:read` - read environment variables
- `config:write` - modify CLI configuration

## 4. Handle Configuration

Read your plugin's configuration via the `PluginConfig` provided by the host:

```rust
let timeout = config.get_or_default("timeout_ms", "1000");
```

## 5. Aliases

Add aliases to a `CommandSpec` with `.with_alias("alias-name")`. Aliases
participate in conflict detection and are shown in help when `show_aliases`
is enabled.
