# Model Registry Guide

The `ModelRegistry` in `ancora-quant` is the central catalogue for locally
available quantized models. This guide explains how to populate, query, and
manage the registry.

## Creating a Registry

```rust
use ancora_quant::registry::ModelRegistry;

// Empty registry.
let mut registry = ModelRegistry::new();

// Registry with a scan root (auto-discovers .gguf/.onnx files).
let mut registry = ModelRegistry::with_scan_root("/opt/models");
```

## Registering Models

### GGUF Models

```rust
use ancora_quant::gguf::{GgufDescriptor, GgufQuantType};

registry.register_gguf("llama3-8b-q4km", GgufDescriptor::new(
    "llama3-8b-q4km",
    "/opt/models/llama3-8b-q4_k_m.gguf",
    "llama",
    8.0,           // param_count_billions
    GgufQuantType::Q4_K,
    4_600_000_000, // file_size_bytes
    8192,          // context_length
));
```

### ONNX Models

```rust
use ancora_quant::onnx::{ExecutionProvider, OnnxDescriptor, OnnxPrecision};

let desc = OnnxDescriptor::new(
    "bert-int8",
    "/opt/models/bert-base-int8.onnx",
    17,
    OnnxPrecision::Int8,
    350_000_000,
    0.11,
)
.with_provider(ExecutionProvider::Cuda)
.with_max_sequence_length(512);

registry.register_onnx("bert-int8", desc);
```

### Capability Flags

After registering a model, attach capability flags:

```rust
use ancora_quant::capability::{Capability, CapabilityBuilder};

let flags = CapabilityBuilder::chat_model(true, 0) // cpu_viable=true
    .with_capability(Capability::CodeGeneration);

registry.set_capabilities("llama3-8b-q4km", flags);
```

## Auto-Discovery

If you set a scan root, call `scan_directory()` to auto-register all `.gguf`
and `.onnx` files found at that path:

```rust
let mut registry = ModelRegistry::with_scan_root("/opt/models");
let count = registry.scan_directory();
println!("Discovered {} models", count);
```

Auto-discovered models get minimal metadata (architecture and quant type are
unknown). Enhance them after discovery by calling `get()` and replacing
the entry via re-registration.

## Querying the Registry

### List all models

```rust
for id in registry.ids() {
    let entry = registry.get(id).unwrap();
    println!("{}: {} format, {:.0} MB RAM",
        id,
        entry.format(),
        entry.estimated_ram_bytes() as f64 / 1024.0 / 1024.0
    );
}
```

### Find models fitting a RAM budget

```rust
let budget = 6 * 1024 * 1024 * 1024; // 6 GB
for (id, entry) in registry.models_fitting_ram(budget) {
    println!("{} fits ({} MB)", id, entry.estimated_ram_bytes() / 1024 / 1024);
}
```

### Sorted by RAM

```rust
for (id, entry) in registry.list_by_ram() {
    println!("{}: {} MB", id, entry.estimated_ram_bytes() / 1024 / 1024);
}
```

## Memory-Aware Selection

Use `ancora_quant::memory::select_model` for automated selection:

```rust
use ancora_quant::memory::{select_model, SelectionPolicy};

let budget = 8 * 1024 * 1024 * 1024;
match select_model(&registry, budget, SelectionPolicy::LargestFit) {
    Some(result) => println!("Selected: {}", result.model_id),
    None => println!("No model fits the budget"),
}
```

## Registry Lifecycle

The registry is an in-memory structure. To persist across restarts, serialize
model descriptors (e.g. to JSON via serde) and reload on startup. A simple
pattern:

1. Load registry from a JSON manifest on startup.
2. Call `scan_directory()` to pick up new files.
3. Save the updated manifest on shutdown.

## Removing Models

```rust
registry.remove("bert-int8");
```

Removing a model also removes its capability flags.

## Integration with RuntimeManager

```rust
use ancora_quant::runtime::RuntimeManager;

let ram = 8 * 1024 * 1024 * 1024;
let mut rt = RuntimeManager::new(ram);
let entry = registry.get("llama3-8b-q4km").expect("model not found");
let handle = rt.load("llama3-8b-q4km", entry)?;
// ... use model ...
rt.unload(handle)?;
```
