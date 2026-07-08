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

pub mod export;
pub mod integrity;
pub mod journal;
pub mod model;
pub mod perf;
pub mod registry;
pub mod runtime;

// Re-export commonly used types.
pub use journal::{SelectionEvent, SelectionJournal};
pub use model::{
    AdapterDescriptor, AdapterFormat, AdapterId, AdapterIntegrity, BaseModel, LoadedAdapter,
    LoraHyperparams,
};
pub use registry::AdapterRegistry;
pub use runtime::{FtError, FtResult};
