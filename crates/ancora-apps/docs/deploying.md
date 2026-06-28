# Deploying a Sample App

## Prerequisites

- Rust 1.70+
- No network access required for any app in this gallery.

## Build

```bash
cargo build -p ancora-apps --release
```

## Run Tests

```bash
cargo test -p ancora-apps
```

All tests run without network calls and pass in air-gapped environments.

## Air-gapped Deployment

The compliance-review app is explicitly validated for air-gapped environments.
To deploy:

1. Copy the compiled binary and the local model files to the target host.
2. Populate the `ComplianceReviewer::government_preset()` rule set (or supply
   custom rules via `ComplianceReviewer::new()`).
3. Call `reviewer.review(artifact_id, text)` and inspect `ReviewResult`.

No DNS resolution, TLS connections, or external API calls are made.

## Local Model Configuration

Each app resolves its model through `local_models::ModelRegistry`. Register
your local model before constructing the app:

```rust
use ancora_apps::local_models::{ModelDescriptor, ModelBackend, ModelRegistry};

let mut registry = ModelRegistry::new();
registry.register(ModelDescriptor::new(
    "my-model",
    ModelBackend::LocalFile { path: "/models/my-model.gguf".to_string() },
    4096,
));
```
