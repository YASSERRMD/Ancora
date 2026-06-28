# CLI Plugin Guide

This guide explains how the ancora-cliplugin system enables CLIs in the Ancora
ecosystem to accept third-party plugins that register commands, integrate with
help output, and respect the permission model.

## Overview

The CLI plugin system consists of several coordinated components:

- **Interface** (`interface.rs`) - defines `CliPlugin`, `CommandSpec`, `ExecContext`, and `ExecOutput`.
- **Registration** (`registration.rs`) - `PluginRegistry` dispatches CLI commands to the right plugin.
- **Discovery** (`discovery.rs`) - scans directories for `plugin.toml` manifests.
- **Help** (`help.rs`) - merges plugin-contributed command sections into the CLI help text.
- **Config** (`config.rs`) - per-plugin key/value configuration with merge support.
- **Permissions** (`permissions.rs`) - scope-based permission enforcement before execution.
- **Update** (`update.rs`) - version comparison and update availability checking.
- **List** (`list.rs`) - renders a tabular list of installed plugins.

## Quick Start

1. Create a struct that implements `CliPlugin`.
2. Return your `PluginMeta` from `meta()`.
3. Declare your commands in `commands()`.
4. Handle invocations in `execute()`.
5. Register your plugin with `PluginRegistry::register()`.

## Command Dispatch

When the user invokes a CLI command, the runtime calls
`PluginRegistry::dispatch(command, ctx)`. The registry resolves the command name
(or alias) to the owning plugin and calls `execute()`.

## Help Integration

After all plugins are registered, call `build_plugin_help_section()` with the
aggregated `CommandSpec` list and pass the result to `compose_help()` alongside
the built-in help string.

## Permission Enforcement

Before executing a sensitive action, call `PermissionEnforcer::check(plugin_id, scope)`.
If it returns `Err(PluginError::PermissionDenied(...))`, surface that error to the
user without performing the action.
