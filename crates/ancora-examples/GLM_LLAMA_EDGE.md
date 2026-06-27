# GLM / llama.cpp Edge Example

Demonstrates configuring multiple GLM model variants and verifying that each
produces a distinct run ID, simulating an edge deployment that cycles through
available model weights.

## What it tests

- `GLM_MODELS` has 4 entries, all distinct, all starting with `"glm-"`
- Each model variant maps to a uniquely named `AgentSpec`
- `Run::generate()` produces a distinct ID for each model iteration

## Pattern

```rust
use ancora_core::run::Run;
use ancora_proto::ancora::AgentSpec;

const GLM_MODELS: &[&str] = &["glm-4", "glm-4-flash", "glm-4-air", "glm-3-turbo"];

let runs: Vec<Run> = (0..GLM_MODELS.len()).map(|_| Run::generate()).collect();
let ids: std::collections::HashSet<&String> = runs.iter().map(|r| &r.id).collect();
assert_eq!(runs.len(), ids.len());
```

## Offline

No network calls. Spec construction and run-ID generation are fully in-process.
