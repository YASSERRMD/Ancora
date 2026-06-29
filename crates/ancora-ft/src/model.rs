//! LoRA adapter descriptor and base model types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Supported adapter formats for export targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterFormat {
    /// LoRA adapter weights in safetensors format.
    LoRaSafetensors,
    /// LoRA adapter weights in GGUF format pointer.
    Gguf,
    /// ONNX model pointer referencing merged weights.
    Onnx,
    /// Raw binary dump (custom).
    Raw,
}

impl std::fmt::Display for AdapterFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterFormat::LoRaSafetensors => write!(f, "lora-safetensors"),
            AdapterFormat::Gguf => write!(f, "gguf"),
            AdapterFormat::Onnx => write!(f, "onnx"),
            AdapterFormat::Raw => write!(f, "raw"),
        }
    }
}

/// A unique identifier for an adapter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdapterId(pub String);

impl AdapterId {
    pub fn new(id: impl Into<String>) -> Self {
        AdapterId(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AdapterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Metadata describing the LoRA rank and alpha hyperparameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoraHyperparams {
    /// LoRA rank (r).
    pub rank: u32,
    /// LoRA scaling factor (alpha).
    pub alpha: f32,
    /// Dropout probability applied during training (0.0 = none).
    pub dropout: f32,
    /// Target module names the adapter was applied to.
    pub target_modules: Vec<String>,
}

impl Default for LoraHyperparams {
    fn default() -> Self {
        LoraHyperparams {
            rank: 8,
            alpha: 16.0,
            dropout: 0.05,
            target_modules: vec!["q_proj".into(), "v_proj".into()],
        }
    }
}

/// Integrity information for an adapter file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterIntegrity {
    /// SHA-256 hex digest of the adapter weight file.
    pub sha256: String,
    /// File size in bytes.
    pub size_bytes: u64,
}

/// Descriptor for a LoRA adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterDescriptor {
    /// Unique identifier.
    pub id: AdapterId,
    /// Human-readable name.
    pub name: String,
    /// Base model this adapter was trained against.
    pub base_model: String,
    /// Filesystem path to the adapter weights.
    pub path: PathBuf,
    /// Adapter format.
    pub format: AdapterFormat,
    /// LoRA hyperparameters.
    pub hyperparams: LoraHyperparams,
    /// Integrity metadata.
    pub integrity: Option<AdapterIntegrity>,
    /// Arbitrary key-value metadata (e.g., task, dataset).
    pub metadata: HashMap<String, String>,
    /// Whether adapter stacking is supported for this adapter.
    pub stackable: bool,
}

impl AdapterDescriptor {
    /// Create a new adapter descriptor with defaults.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        base_model: impl Into<String>,
        path: PathBuf,
    ) -> Self {
        AdapterDescriptor {
            id: AdapterId::new(id),
            name: name.into(),
            base_model: base_model.into(),
            path,
            format: AdapterFormat::LoRaSafetensors,
            hyperparams: LoraHyperparams::default(),
            integrity: None,
            metadata: HashMap::new(),
            stackable: true,
        }
    }

    /// Set integrity info.
    pub fn with_integrity(mut self, integrity: AdapterIntegrity) -> Self {
        self.integrity = Some(integrity);
        self
    }

    /// Set format.
    pub fn with_format(mut self, format: AdapterFormat) -> Self {
        self.format = format;
        self
    }

    /// Mark as not stackable.
    pub fn not_stackable(mut self) -> Self {
        self.stackable = false;
        self
    }
}

/// A loaded adapter ready to be applied on top of a base model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedAdapter {
    /// The descriptor used to load this adapter.
    pub descriptor: AdapterDescriptor,
    /// Simulated weight data (in a real system, this would be tensors).
    pub weight_bytes: u64,
    /// Whether the adapter is currently active.
    pub active: bool,
}

impl LoadedAdapter {
    pub fn new(descriptor: AdapterDescriptor, weight_bytes: u64) -> Self {
        LoadedAdapter {
            descriptor,
            weight_bytes,
            active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }
}

/// A base model onto which adapters can be loaded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseModel {
    /// Model identifier (e.g., "llama-3.1-8b").
    pub id: String,
    /// Path to model weights.
    pub path: PathBuf,
    /// Parameter count in billions (approximate).
    pub params_billions: f32,
    /// Currently loaded adapters, in stack order.
    pub loaded_adapters: Vec<LoadedAdapter>,
}

impl BaseModel {
    pub fn new(id: impl Into<String>, path: PathBuf, params_billions: f32) -> Self {
        BaseModel {
            id: id.into(),
            path,
            params_billions,
            loaded_adapters: Vec::new(),
        }
    }

    /// Load an adapter onto the model.
    pub fn load_adapter(&mut self, adapter: LoadedAdapter) {
        self.loaded_adapters.push(adapter);
    }

    /// Return the number of active adapters.
    pub fn active_adapter_count(&self) -> usize {
        self.loaded_adapters.iter().filter(|a| a.active).count()
    }

    /// Get adapter by id.
    pub fn get_adapter(&self, id: &AdapterId) -> Option<&LoadedAdapter> {
        self.loaded_adapters
            .iter()
            .find(|a| &a.descriptor.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_descriptor_defaults() {
        let desc = AdapterDescriptor::new(
            "a1",
            "Test Adapter",
            "llama-3.1-8b",
            PathBuf::from("/tmp/adapter.safetensors"),
        );
        assert_eq!(desc.id.as_str(), "a1");
        assert_eq!(desc.base_model, "llama-3.1-8b");
        assert!(desc.stackable);
        assert!(desc.integrity.is_none());
        assert_eq!(desc.hyperparams.rank, 8);
    }

    #[test]
    fn adapter_id_display() {
        let id = AdapterId::new("my-adapter");
        assert_eq!(id.to_string(), "my-adapter");
    }

    #[test]
    fn base_model_load_adapter() {
        let mut model =
            BaseModel::new("llama-3.1-8b", PathBuf::from("/tmp/model"), 8.0);
        let desc = AdapterDescriptor::new(
            "a1",
            "Test",
            "llama-3.1-8b",
            PathBuf::from("/tmp/a.safetensors"),
        );
        let loaded = LoadedAdapter::new(desc, 1024);
        model.load_adapter(loaded);
        assert_eq!(model.active_adapter_count(), 1);
    }

    #[test]
    fn loaded_adapter_deactivate() {
        let desc = AdapterDescriptor::new(
            "a1",
            "Test",
            "llama-3.1-8b",
            PathBuf::from("/tmp/a.safetensors"),
        );
        let mut loaded = LoadedAdapter::new(desc, 512);
        assert!(loaded.active);
        loaded.deactivate();
        assert!(!loaded.active);
        loaded.activate();
        assert!(loaded.active);
    }

    #[test]
    fn format_display() {
        assert_eq!(AdapterFormat::Gguf.to_string(), "gguf");
        assert_eq!(AdapterFormat::Onnx.to_string(), "onnx");
    }
}
