# Ecosystem Test Plan

## Scope

This document describes the end-to-end test plan for the Ancora extension ecosystem.

## Goals

- All extension lifecycle operations must work offline (no network required).
- Plugins are sandboxed; a crash in one plugin does not affect others.
- Residency is enforced: each plugin runs in an identifiable environment context.
- Trust policy gates installation of low-trust extensions.
- Air-gapped registry workflow supports closed-network deployments.

## Test Areas

| Area | Test File | Description |
|------|-----------|-------------|
| Plugin authoring | test_plugin_from_template | Author a plugin from a template |
| Interop kit | test_plugin_interop_kit | Plugin passes the interop kit checks |
| Registry publish | test_publish_to_registry | Publish a plugin to a local registry |
| Registry install | test_install_from_registry | Install a plugin from the registry |
| Sandboxing | test_sandboxed | Plugin runs sandboxed |
| Catalog | test_catalog_install | Catalog install adds a tool |
| Graph builder | test_builder_graph | Graph builder produces a runnable graph |
| CLI plugin | test_cli_plugin | CLI plugin registers and runs commands |
| Adapter | test_adapter_tool | Framework adapter imports a tool |
| Recipe | test_recipe_runs | Recipe installs and runs to completion |
| Trust | test_trust_blocks | Trust policy blocks low-trust installs |
| Air-gap | test_airgap | Air-gapped registry workflow works offline |
| Parity | test_parity | Extension parity across language runtimes |
| All offline | test_all_offline | Full ecosystem works without network |
| Crash isolation | test_crash_isolated | Plugin crash does not affect siblings |

## Acceptance Criteria

- All tests pass with `cargo test -p ancora-eco-e2e`.
- Zero network calls during test execution.
- Residency IDs are stable across runs.
