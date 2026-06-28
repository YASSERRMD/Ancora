# Distributing CLI Plugins

This document describes how to package and distribute a CLI plugin for Ancora.

## Directory Layout

A distributable plugin is a directory containing:

```
my-plugin/
  plugin.toml       # manifest (required)
  README.md         # user-facing documentation
  src/              # Rust source (if distributed as source)
  Cargo.toml        # (if distributed as source)
```

## The plugin.toml Manifest

The manifest is parsed by `ancora_cliplugin::discovery`. Required fields:

| Field     | Type   | Description                |
|-----------|--------|----------------------------|
| `id`      | string | Unique reverse-DNS identifier (e.g. `acme.myplugin`) |
| `name`    | string | Human-readable display name |
| `version` | string | Semantic version (MAJOR.MINOR.PATCH) |

Optional fields:

| Field        | Type   | Description                     |
|--------------|--------|---------------------------------|
| `description`| string | Short description for discovery |
| `author`     | string | Author name or email            |
| `update_url` | string | URL for the update registry     |

## Installation Paths

Ancora discovers plugins in the following order (highest priority first):

1. **Explicit** - paths provided via `--plugin <path>` on the CLI.
2. **User** - `$HOME/.ancora/plugins/` (platform default user plugin directory).
3. **System** - `/usr/local/share/ancora/plugins/` (system-wide installation).

## Version Management

Plugins should follow semantic versioning. The update system compares
the installed version string against the registry entry using the
`Version` type in `ancora_cliplugin::update`. Breaking changes must
increment the major version.

## Conflict Avoidance

Choose command names that are unlikely to conflict with built-in commands
or other plugins. Use the reverse-DNS prefix of your plugin id as a
namespace hint, e.g., `acme-` prefix for all commands from an Acme Corp
plugin.

## Update Registry

To publish updates, provide an `update_url` in your manifest that returns a
JSON object with `latest_version` and `notes` fields. The host CLI will poll
this URL and surface update notifications to the user via the update status
system.
