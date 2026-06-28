# Plugin Registry

The Ancora plugin registry is the authoritative index of published plugins.

## Querying the registry

Use the `ancora registry search <keyword>` command to find plugins.

## Submitting a plugin

1. Publish your crate to crates.io.
2. Open a PR adding your `ancora-catalog.toml` entry to the `registry/` folder.
3. CI will validate the entry and run interop checks.
4. Once approved, the plugin appears in registry search results.
