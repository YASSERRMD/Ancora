# Quantized Model Management

`ancora-quant` provides a complete toolkit for managing quantized local models
in the Ancora agent framework. It handles GGUF and ONNX model descriptors,
quantization level metadata, file integrity, a local registry, runtime
load/unload, memory-aware selection, capability flags, and prefetch/caching.

## Supported Formats

### GGUF

GGUF (GPT-Generated Unified Format) is the standard format used by llama.cpp
and compatible runtimes. It bundles model weights, architecture metadata, and
tokenizer data in a single file.

Key properties captured per model:
- Architecture (llama, mistral, phi, gemma, etc.)
- Quantization type (Q4_K_M, Q5_K_S, Q8_0, F16, etc.)
- Parameter count in billions
- Context length
- File size

### ONNX

ONNX (Open Neural Network Exchange) is a cross-platform format supported by
ONNX Runtime, TensorRT, DirectML, CoreML, and OpenVino. Quantized ONNX models
use INT8, INT4, or FP16 precision.

Key properties captured per model:
- Opset version
- Precision (fp32, fp16, bf16, int8, int4, mixed)
- Execution provider preference order
- Maximum sequence length

## Core Components

### Quantization Level (`quant_level`)

Provides `QuantTier` (coarse tier enum) and `QuantLevel` (detailed metadata
including bits-per-weight, grouped-quant flag, and trade-off notes).

Use `QuantLevel::recommend_for_ram_gb` to automatically select a tier given
available RAM and model parameter count.

### File Integrity (`integrity`)

Verifies model files using Adler-32 (built-in), SHA-256, or MD5 checksums.
Detects corruption, incomplete downloads, and file size mismatches.

### Local Registry (`registry`)

`ModelRegistry` maintains an in-memory catalogue of models. Supports:
- Registration of GGUF and ONNX models
- Capability flag assignment per model
- RAM-aware filtering
- Directory scanning for auto-discovery

### Runtime Management (`runtime`)

`RuntimeManager` simulates load/unload operations against a RAM budget:
- Tracks live model handles
- Prevents loading beyond available RAM
- Supports multi-instance loading
- LRU eviction helper

### Memory-Aware Selection (`memory`)

`select_model` chooses the best model for a RAM budget using configurable
policies: `LargestFit`, `SmallestFit`, or `MostCompressed`.

`select_model_with_headroom` reserves a fraction of available RAM before
selection.

### Capability Flags (`capability`)

`CapabilityFlags` records what tasks a model supports (chat, code, embedding,
vision, tool-calling, etc.) and quantization-imposed limitations such as
reduced context length or CPU viability.

### Prefetch and Cache (`prefetch`)

`PrefetchCache` manages a prefetch queue and in-memory cache state:
- Enqueue models for background loading
- LRU, LFU, or priority-based eviction
- Touch-based access tracking
- Manual eviction

## Usage Pattern

```rust
use ancora_quant::registry::ModelRegistry;
use ancora_quant::gguf::{GgufDescriptor, GgufQuantType};
use ancora_quant::memory::{select_model, SelectionPolicy};
use ancora_quant::runtime::RuntimeManager;

let mut registry = ModelRegistry::new();
registry.register_gguf("llama3-8b-q4", GgufDescriptor::new(
    "llama3-8b-q4",
    "/models/llama3-8b-q4_k_m.gguf",
    "llama",
    8.0,
    GgufQuantType::Q4_K,
    4_500_000_000,
    8192,
));

// Select best model for 6 GB RAM budget.
let budget = 6 * 1024 * 1024 * 1024;
if let Some(result) = select_model(&registry, budget, SelectionPolicy::LargestFit) {
    let mut rt = RuntimeManager::new(budget);
    let handle = rt.load(result.model_id, result.entry).expect("load");
    // ... run inference ...
    rt.unload(handle).expect("unload");
}
```
