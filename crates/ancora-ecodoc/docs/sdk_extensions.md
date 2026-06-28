# SDK Extension Ergonomics

The `sdk_extensions` module provides a fluent builder API to reduce boilerplate.

## PluginMetaBuilder

```rust
use ancora_ecodoc::sdk_extensions::PluginMetaBuilder;

let meta = PluginMetaBuilder::new()
    .name("my-plugin")
    .version("0.1.0")
    .author("Alice")
    .description("Does something useful")
    .build()?;
```

## quick_meta

For simple cases where all fields are known at compile time:

```rust
use ancora_ecodoc::sdk_extensions::quick_meta;

let meta = quick_meta("my-plugin", "0.1.0", "Alice", "Does something useful");
```
