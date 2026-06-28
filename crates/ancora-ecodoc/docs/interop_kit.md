# Interop Test Kit

The interop kit provides helpers to verify that your plugin correctly implements the Ancora runtime contract.

## Running interop checks

Add the interop checks to your test suite:

```rust
use ancora_ecodoc::interop_kit::{InteropSuite, InteropCheck, check_plugin_name_not_empty};

#[test]
fn interop() {
    let suite = InteropSuite::new().add(InteropCheck {
        name: "plugin-name",
        description: "Plugin name is non-empty",
        run: || check_plugin_name_not_empty("my-plugin"),
    });
    assert!(suite.all_pass(), "interop failures: {:?}", suite.run_all());
}
```

All checks run offline and require no network access.
