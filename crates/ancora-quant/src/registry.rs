/// Local model registry.
///
/// Maintains an in-memory catalogue of locally available quantized models
/// indexed by a string ID. Models can be GGUF or ONNX.
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::capability::CapabilityFlags;
use crate::gguf::GgufDescriptor;
use crate::onnx::OnnxDescriptor;
use crate::quant_level::QuantTier;

/// A model entry in the local registry.
#[derive(Debug, Clone)]
pub enum ModelEntry {
    Gguf(GgufDescriptor),
    Onnx(OnnxDescriptor),
}

impl ModelEntry {
    /// Human-readable name.
    pub fn name(&self) -> &str {
        match self {
            ModelEntry::Gguf(d) => &d.name,
            ModelEntry::Onnx(d) => &d.name,
        }
    }

    /// Local file path.
    pub fn path(&self) -> &Path {
        match self {
            ModelEntry::Gguf(d) => &d.path,
            ModelEntry::Onnx(d) => &d.path,
        }
    }

    /// File size in bytes.
    pub fn file_size_bytes(&self) -> u64 {
        match self {
            ModelEntry::Gguf(d) => d.file_size_bytes,
            ModelEntry::Onnx(d) => d.file_size_bytes,
        }
    }

    /// Estimated RAM requirement in bytes.
    pub fn estimated_ram_bytes(&self) -> u64 {
        match self {
            ModelEntry::Gguf(d) => d.estimated_ram_bytes(),
            ModelEntry::Onnx(d) => d.estimated_ram_bytes(),
        }
    }

    /// Parameter count in billions.
    pub fn param_count_billions(&self) -> f32 {
        match self {
            ModelEntry::Gguf(d) => d.param_count_billions,
            ModelEntry::Onnx(d) => d.param_count_billions,
        }
    }

    /// Format tag (e.g. "gguf", "onnx").
    pub fn format(&self) -> &'static str {
        match self {
            ModelEntry::Gguf(_) => "gguf",
            ModelEntry::Onnx(_) => "onnx",
        }
    }
}

/// Local model registry.
#[derive(Debug, Default)]
pub struct ModelRegistry {
    /// Map from model ID to entry.
    entries: HashMap<String, ModelEntry>,
    /// Capability flags per model ID.
    capabilities: HashMap<String, CapabilityFlags>,
    /// Scan root directory (optional).
    scan_root: Option<PathBuf>,
}

impl ModelRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        ModelRegistry::default()
    }

    /// Create a registry with a scan root directory.
    pub fn with_scan_root(path: impl Into<PathBuf>) -> Self {
        ModelRegistry {
            scan_root: Some(path.into()),
            ..Default::default()
        }
    }

    /// Register a GGUF model.
    pub fn register_gguf(&mut self, id: impl Into<String>, desc: GgufDescriptor) {
        self.entries.insert(id.into(), ModelEntry::Gguf(desc));
    }

    /// Register an ONNX model.
    pub fn register_onnx(&mut self, id: impl Into<String>, desc: OnnxDescriptor) {
        self.entries.insert(id.into(), ModelEntry::Onnx(desc));
    }

    /// Set capability flags for a registered model.
    pub fn set_capabilities(&mut self, id: impl Into<String>, flags: CapabilityFlags) {
        self.capabilities.insert(id.into(), flags);
    }

    /// Look up a model by ID.
    pub fn get(&self, id: &str) -> Option<&ModelEntry> {
        self.entries.get(id)
    }

    /// Get capability flags for a model.
    pub fn capabilities(&self, id: &str) -> Option<&CapabilityFlags> {
        self.capabilities.get(id)
    }

    /// Remove a model from the registry.
    pub fn remove(&mut self, id: &str) -> Option<ModelEntry> {
        self.capabilities.remove(id);
        self.entries.remove(id)
    }

    /// Iterate over all registered model IDs.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.entries.keys().map(|s| s.as_str())
    }

    /// Number of registered models.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// List all models sorted by estimated RAM requirement (ascending).
    pub fn list_by_ram(&self) -> Vec<(&str, &ModelEntry)> {
        let mut v: Vec<(&str, &ModelEntry)> =
            self.entries.iter().map(|(k, v)| (k.as_str(), v)).collect();
        v.sort_by_key(|(_, e)| e.estimated_ram_bytes());
        v
    }

    /// Find all models that fit within a RAM budget (bytes).
    pub fn models_fitting_ram(&self, ram_bytes: u64) -> Vec<(&str, &ModelEntry)> {
        self.entries
            .iter()
            .filter(|(_, e)| e.estimated_ram_bytes() <= ram_bytes)
            .map(|(k, e)| (k.as_str(), e))
            .collect()
    }

    /// Scan root directory for .gguf and .onnx files and register them.
    ///
    /// This is a best-effort scan; files that cannot be stat'd are skipped.
    pub fn scan_directory(&mut self) -> usize {
        let root = match &self.scan_root {
            Some(r) => r.clone(),
            None => return 0,
        };
        let mut count = 0;
        if let Ok(entries) = std::fs::read_dir(&root) {
            for entry in entries.flatten() {
                let path = entry.path();
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                if ext == "gguf" {
                    use crate::gguf::{GgufDescriptor, GgufQuantType};
                    let desc = GgufDescriptor::new(
                        stem.clone(),
                        path,
                        "unknown",
                        0.0,
                        GgufQuantType::Other("unknown".into()),
                        size,
                        2048,
                    );
                    self.register_gguf(stem, desc);
                    count += 1;
                } else if ext == "onnx" {
                    use crate::onnx::{OnnxDescriptor, OnnxPrecision};
                    let desc = OnnxDescriptor::new(stem.clone(), path, 17, OnnxPrecision::Float32, size, 0.0);
                    self.register_onnx(stem, desc);
                    count += 1;
                }
            }
        }
        count
    }

    /// Return the scan root, if set.
    pub fn scan_root(&self) -> Option<&Path> {
        self.scan_root.as_deref()
    }
}
