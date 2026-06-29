/// ancora-quant: quantized local model management.
///
/// Provides GGUF and ONNX model descriptors, quantization level metadata,
/// file integrity verification, a local model registry, model load/unload,
/// memory-aware selection, capability flags, and prefetch/cache management.

pub mod capability;
pub mod gguf;
pub mod integrity;
pub mod memory;
pub mod onnx;
pub mod prefetch;
pub mod quant_level;
pub mod registry;
pub mod runtime;
pub mod tradeoff;

#[cfg(test)]
mod tests;
