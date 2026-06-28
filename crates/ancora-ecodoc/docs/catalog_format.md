# Catalog Format

Each plugin crate must include an `ancora-catalog.toml` at the crate root.

## Schema (v1)

```toml
schema_version = 1
name = "my-plugin"
version = "0.1.0"
description = "Short description of my plugin"
author = "Your Name <you@example.com>"
license = "MIT OR Apache-2.0"
keywords = ["example", "plugin"]
```

## Validation

Run `ancora catalog validate` to check your manifest before publishing.
All fields except `keywords` are required.
