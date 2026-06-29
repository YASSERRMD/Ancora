//! ancora-ft: Local fine-tuning and LoRA adapter loading for the Ancora agent framework.
//!
//! Provides:
//! - [`model`]: adapter descriptor, base model, loaded adapter types
//! - [`runtime`]: load, hot-swap, stack, and per-tenant selection
//! - [`integrity`]: checksum-based adapter integrity verification
//! - [`registry`]: catalog of known adapters
//! - [`export`]: export adapters to GGUF/ONNX pointer records
//! - [`journal`]: journaled and replayable adapter selection
//! - [`perf`]: adapter performance notes

pub mod model;
pub mod runtime;
pub mod integrity;
pub mod registry;
pub mod export;
pub mod journal;
pub mod perf;

// Re-export commonly used types.
pub use model::{
    AdapterDescriptor, AdapterFormat, AdapterId, AdapterIntegrity, BaseModel,
    LoadedAdapter, LoraHyperparams,
};
pub use runtime::{FtError, FtResult};
pub use registry::AdapterRegistry;
pub use journal::{SelectionJournal, SelectionEvent};
