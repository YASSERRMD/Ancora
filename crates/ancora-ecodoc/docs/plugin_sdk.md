# Plugin SDK

The Plugin SDK (`ancora-sdk`) provides the `Plugin` trait and associated types.

## Minimum implementation

```rust
use ancora_sdk::{Plugin, PluginMeta, PluginEvent, PluginResult};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "my-plugin".into(),
            version: "0.1.0".into(),
            author: "You".into(),
            description: "My first plugin".into(),
        }
    }

    fn on_event(&self, _event: &PluginEvent) -> PluginResult<()> {
        Ok(())
    }
}
```

## SDK version

The current SDK version is `0.1.0`. Plugins must target this version or later.
