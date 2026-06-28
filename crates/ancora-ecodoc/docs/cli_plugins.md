# CLI Plugins

Ancora allows plugins to contribute additional subcommands to the `ancora` binary.

## Registering a CLI command

In your plugin's `on_event(Init)` handler, register a `CliCommand`:

```rust
fn on_event(&self, event: &PluginEvent) -> PluginResult<()> {
    if let PluginEvent::Init = event {
        CLI_REGISTRY.add(CliCommand {
            name: "my-cmd".into(),
            about: "Does my thing".into(),
            usage: "ancora my-cmd [OPTIONS]".into(),
        })?;
    }
    Ok(())
}
```

Commands must be registered before the CLI registry is frozen (during `Init`).
