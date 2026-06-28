# Upgrade Notes

## Upgrading from 0.5.x to 0.6.0 (breaking)

1. Update plugin manifest to schema v3
2. Rename `PluginCtx::invoke` to `PluginCtx::call`
3. Remove deprecated `ancora_plugin::v1` imports
4. Re-run `cargo build` and resolve any type errors

## Upgrading from 0.6.0 to 0.6.x (non-breaking)

1. Run `cargo update` to pick up patch releases
