# Extension Author Quickstart

Get a working Ancora plugin published in six steps.

## Step 1 - Create a new crate

```bash
cargo new --lib my-ancora-plugin
cd my-ancora-plugin
```

## Step 2 - Add the Ancora SDK dependency

Add to `Cargo.toml`:

```toml
[dependencies]
ancora-sdk = "0.1"
```

## Step 3 - Implement the Plugin trait

In `src/lib.rs`:

```rust
use ancora_sdk::{Plugin, PluginMeta, PluginEvent, PluginResult};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "my-ancora-plugin".into(),
            version: "0.1.0".into(),
            author: "You".into(),
            description: "My first Ancora plugin".into(),
        }
    }

    fn on_event(&self, _event: &PluginEvent) -> PluginResult<()> {
        Ok(())
    }
}
```

## Step 4 - Write tests

```bash
cargo test
```

## Step 5 - Add catalog entry

Create `ancora-catalog.toml` in the crate root with the fields from the Catalog Format doc.

## Step 6 - Publish

```bash
cargo publish
```

Your plugin is now discoverable via `ancora registry search`.
