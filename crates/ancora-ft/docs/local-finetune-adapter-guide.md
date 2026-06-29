# Local Fine-Tuning and Adapter Guide

This guide explains how to use `ancora-ft` to load and manage LoRA adapters
and local fine-tunes within the Ancora agent framework.

## Overview

`ancora-ft` provides a pure-Rust, offline-capable subsystem for:

- Describing LoRA adapters with hyperparameters and integrity metadata
- Loading adapters onto a base model
- Hot-swapping the active adapter without restarting the model process
- Stacking multiple compatible adapters
- Verifying adapter integrity via checksums
- Selecting adapters per-tenant with full journal and replay support
- Cataloging adapters in a typed registry
- Exporting fine-tuned adapters to GGUF and ONNX pointer records

## Quick Start

```rust
use ancora_ft::{AdapterDescriptor, BaseModel};
use ancora_ft::integrity::attach_integrity;
use ancora_ft::runtime::load_adapter_onto;
use std::path::PathBuf;

let mut model = BaseModel::new("llama-3.1-8b", PathBuf::from("/models/llama"), 8.0);
let mut adapter = AdapterDescriptor::new(
    "my-adapter", "My LoRA", "llama-3.1-8b",
    PathBuf::from("/adapters/my.safetensors"),
);
attach_integrity(&mut adapter, 1_048_576);
let id = load_adapter_onto(&mut model, adapter, 1_048_576).unwrap();
println!("Loaded: {}", id);
```

## Adapter Descriptor

An `AdapterDescriptor` captures all metadata about a LoRA adapter:

| Field | Type | Description |
|-------|------|-------------|
| `id` | `AdapterId` | Unique identifier |
| `name` | `String` | Human-readable name |
| `base_model` | `String` | Base model the adapter targets |
| `path` | `PathBuf` | Path to adapter weight file |
| `format` | `AdapterFormat` | Weight format (LoRA safetensors, GGUF, ONNX, Raw) |
| `hyperparams` | `LoraHyperparams` | Rank, alpha, dropout, target modules |
| `integrity` | `Option<AdapterIntegrity>` | SHA-256 checksum and size |
| `metadata` | `HashMap<String, String>` | Arbitrary key-value tags |
| `stackable` | `bool` | Whether adapter can be stacked |

## Loading and Hot-Swap

```rust
use ancora_ft::runtime::{load_adapter_onto, hot_swap};

// Load (compatible with model.id == adapter.base_model)
load_adapter_onto(&mut model, adapter, weight_bytes)?;

// Hot-swap: deactivates all existing adapters, loads new one
hot_swap(&mut model, new_adapter, weight_bytes)?;
```

## Stacking

When multiple adapters are stackable, they can be applied in order:

```rust
use ancora_ft::runtime::stack_adapters;

let ids = stack_adapters(&mut model, vec![
    (adapter_a, bytes_a),
    (adapter_b, bytes_b),
])?;
```

## Integrity Verification

```rust
use ancora_ft::integrity::{attach_integrity, verify_integrity, verify_all};

attach_integrity(&mut descriptor, file_size_bytes);
verify_integrity(&descriptor)?;  // returns Err on mismatch
```

## Per-Tenant Selection

```rust
use ancora_ft::journal::{SelectionJournal, select_for_tenant};
use ancora_ft::runtime::TenantAdapterMap;

let mut journal = SelectionJournal::new();
let mut map = TenantAdapterMap::new();
select_for_tenant(&mut journal, &mut map, "tenant-a", adapter_id);
let replayed = journal.replay();  // reconstruct from log
```

## Export to GGUF/ONNX

```rust
use ancora_ft::export::{export_adapter, ExportOptions};
use std::path::PathBuf;

let opts = ExportOptions::gguf(PathBuf::from("/exports"), "Q4_K_M");
let result = export_adapter(&descriptor, "llama-3.1-8b", &opts)?;
println!("GGUF at: {:?}", result.path());

let opts2 = ExportOptions::onnx(PathBuf::from("/exports"), 17);
let result2 = export_adapter(&descriptor, "llama-3.1-8b", &opts2)?;
println!("ONNX at: {:?}", result2.path());
```
