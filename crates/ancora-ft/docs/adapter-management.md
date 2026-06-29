# Adapter Management

This document covers operational management of LoRA adapters in `ancora-ft`.

## Adapter Registry

The `AdapterRegistry` is a typed catalog of known adapters. It supports:

- `register(descriptor)` -- add a new adapter (returns `Err` if id already exists)
- `upsert(descriptor)` -- add or replace unconditionally
- `get(id)` -- retrieve by id
- `remove(id)` -- remove by id
- `list_ids()` -- sorted list of all registered ids
- `by_base_model(base_model)` -- filter by base model

```rust
use ancora_ft::AdapterRegistry;

let mut registry = AdapterRegistry::new();
registry.register(descriptor)?;
for id in registry.list_ids() {
    println!("{}", id);
}
```

## Lifecycle

```
AdapterDescriptor (defined)
    |
    v
attach_integrity (checksum stamped)
    |
    v
AdapterRegistry.register (catalogued)
    |
    v
load_adapter_onto / hot_swap / stack_adapters (activated on model)
    |
    v
export_adapter (GGUF/ONNX pointer for distribution)
```

## Error Handling

All operations return `FtResult<T>` which is `Result<T, FtError>`. Variants:

| Variant | When |
|---------|------|
| `AdapterNotFound` | Requested id not in registry or model |
| `IncompatibleBaseModel` | Adapter targets a different base model |
| `StackingNotSupported` | Adapter is not stackable |
| `IntegrityFailure` | Checksum mismatch or missing integrity metadata |
| `RegistryConflict` | Duplicate id on register |
| `TenantNotFound` | Tenant id has no assignment |
| `ExportError` | Unsupported format or conversion issue |

## Performance Notes

Use `AdapterPerfNote` to record observed overhead per adapter:

```rust
use ancora_ft::perf::{AdapterPerfNote, PerfNoteRegistry};

let note = AdapterPerfNote::new(adapter_id)
    .with_latency(12.5)
    .with_memory(64.0)
    .with_throughput_factor(0.97)
    .with_notes("Tested on H100 with batch size 8");

let mut perf_reg = PerfNoteRegistry::new();
perf_reg.record(note);
let acceptable = perf_reg.acceptable_adapters();
```

Heuristic thresholds: latency overhead < 50ms AND throughput factor > 0.8.
