# Plugin SDK Overview

The `ancora-plugin` crate provides a stable, versioned extension point system for the Ancora agent framework. Plugins are self-contained Rust crates that implement one of the defined extension-point traits, declare a manifest describing their identity and SDK compatibility, and declare the permission scopes they require.

## Core Concepts

- **Manifest** (`manifest` module): Every plugin ships a `PluginManifest` that specifies a unique id, a human-readable name, the plugin version, the SDK version range it supports, the extension point kind, and the permission scopes it requires.

- **Extension points** (`extension_points` module): Enumerate all stable hooks a plugin can attach to - provider, vector-store, tool, memory, guardrail, grader, and exporter.

- **Compatibility** (`compatibility` module): Before loading a plugin the framework checks that the running SDK version falls within the manifest's declared `[min_sdk, max_sdk]` range.

- **Permission scoping** (`permission` module): Each plugin instance is granted only the scopes it declared. Calls that require an ungranted scope are rejected at runtime.

- **Discovery** (`discovery` module): The `PluginRegistry` keeps track of all loaded manifests and allows lookup by id or by kind.

## Stability

All public types and traits in this crate follow semantic versioning. Breaking changes will increment the SDK major version, minor additions increment the minor version. Plugins declare the range they support and are rejected if the running SDK is outside that range.
