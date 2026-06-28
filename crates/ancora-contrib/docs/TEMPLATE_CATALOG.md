# Template catalog

`ancora-contrib` ships one template module for every extension point in the
ancora plugin SDK. Each template is a self-contained Rust module that compiles
without external dependencies and can be used immediately as a starting point.

| Extension point | Source file | Trait to implement | Struct to rename |
|----------------|-------------|-------------------|-----------------|
| Provider | `src/provider_template.rs` | `ProviderAdapter` | `MyProvider` |
| Vector store | `src/vectorstore_template.rs` | `VectorStoreAdapter` | `MyVectorStore` |
| Tool | `src/tool_template.rs` | `ToolPlugin` | `MyTool` |
| Grader | `src/grader_template.rs` | `GraderPlugin` | `MyGrader` |
| Guardrail | `src/guardrail_template.rs` | `GuardrailPlugin` | `MyGuardrail` |
| Exporter | `src/exporter_template.rs` | `ExporterPlugin` | `MyExporter` |
| Full plugin | `src/plugin_template.rs` | `Plugin` | `MyPlugin` |

## Scaffolding command

Use the `scaffolding::scaffold` function to generate a ready-to-compile crate
skeleton for any extension kind:

```rust
use ancora_contrib::scaffolding::{ScaffoldKind, ScaffoldRequest, scaffold};

let req = ScaffoldRequest::new(ScaffoldKind::Provider, "AcmeCloud")
    .with_author("alice");
let output = scaffold(&req).unwrap();
// output.files contains Cargo.toml, src/lib.rs, src/acme_cloud.rs,
// src/tests.rs, src/conformance.rs, docs/README.md
```

## Conformance harness

Every extension kind has a built-in conformance suite:

```rust
use ancora_contrib::conformance::provider_suite;
use std::sync::Arc;

let suite = provider_suite(Arc::new(MyProvider::new("key")));
let report = suite.run();
assert!(report.all_passed(), "{report}");
```

See `src/conformance.rs` for all available suite constructors.
