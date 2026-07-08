//! Local model preload on boot for headless OS integration.
//!
//! Manages model manifests, preload state, and cache validation
//! so the agent can serve inference immediately after boot with no
//! network round-trips.

use std::collections::HashMap;
use std::time::Duration;

/// Supported quantization levels for local model files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quantization {
    F32,
    F16,
    Q8,
    Q4,
    Q2,
}

impl std::fmt::Display for Quantization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quantization::F32 => write!(f, "f32"),
            Quantization::F16 => write!(f, "f16"),
            Quantization::Q8 => write!(f, "q8"),
            Quantization::Q4 => write!(f, "q4"),
            Quantization::Q2 => write!(f, "q2"),
        }
    }
}

/// Descriptor for a model stored on local disk.
#[derive(Debug, Clone)]
pub struct ModelDescriptor {
    pub id: String,
    pub path: String,
    pub size_bytes: u64,
    pub quantization: Quantization,
    pub context_length: usize,
    pub sha256: Option<String>,
}

impl ModelDescriptor {
    pub fn new(id: impl Into<String>, path: impl Into<String>, size_bytes: u64) -> Self {
        ModelDescriptor {
            id: id.into(),
            path: path.into(),
            size_bytes,
            quantization: Quantization::Q4,
            context_length: 4096,
            sha256: None,
        }
    }

    pub fn with_quantization(mut self, q: Quantization) -> Self {
        self.quantization = q;
        self
    }

    pub fn with_context_length(mut self, ctx: usize) -> Self {
        self.context_length = ctx;
        self
    }

    pub fn with_sha256(mut self, hash: impl Into<String>) -> Self {
        self.sha256 = Some(hash.into());
        self
    }

    /// Returns the model size in megabytes.
    pub fn size_mb(&self) -> u64 {
        self.size_bytes / (1024 * 1024)
    }
}

/// Preload state for a single model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreloadState {
    Pending,
    Loading,
    Loaded,
    Failed(String),
    Evicted,
}

impl std::fmt::Display for PreloadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreloadState::Pending => write!(f, "pending"),
            PreloadState::Loading => write!(f, "loading"),
            PreloadState::Loaded => write!(f, "loaded"),
            PreloadState::Failed(e) => write!(f, "failed: {}", e),
            PreloadState::Evicted => write!(f, "evicted"),
        }
    }
}

/// Record of a model preload attempt.
#[derive(Debug, Clone)]
pub struct PreloadRecord {
    pub model_id: String,
    pub state: PreloadState,
    pub duration: Duration,
    pub memory_mb: u64,
}

impl PreloadRecord {
    pub fn success(model_id: impl Into<String>, duration: Duration, memory_mb: u64) -> Self {
        PreloadRecord {
            model_id: model_id.into(),
            state: PreloadState::Loaded,
            duration,
            memory_mb,
        }
    }

    pub fn failure(
        model_id: impl Into<String>,
        duration: Duration,
        reason: impl Into<String>,
    ) -> Self {
        PreloadRecord {
            model_id: model_id.into(),
            state: PreloadState::Failed(reason.into()),
            duration,
            memory_mb: 0,
        }
    }
}

/// Registry of all models configured for preload on boot.
pub struct ModelRegistry {
    models: HashMap<String, ModelDescriptor>,
    states: HashMap<String, PreloadState>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        ModelRegistry {
            models: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub fn register(&mut self, desc: ModelDescriptor) {
        let id = desc.id.clone();
        self.models.insert(id.clone(), desc);
        self.states.insert(id, PreloadState::Pending);
    }

    pub fn get(&self, id: &str) -> Option<&ModelDescriptor> {
        self.models.get(id)
    }

    pub fn state(&self, id: &str) -> Option<&PreloadState> {
        self.states.get(id)
    }

    pub fn set_state(&mut self, id: &str, state: PreloadState) {
        self.states.insert(id.to_string(), state);
    }

    pub fn loaded_count(&self) -> usize {
        self.states
            .values()
            .filter(|s| **s == PreloadState::Loaded)
            .count()
    }

    pub fn all_loaded(&self) -> bool {
        !self.states.is_empty() && self.states.values().all(|s| *s == PreloadState::Loaded)
    }

    pub fn ids(&self) -> Vec<&str> {
        self.models.keys().map(|s| s.as_str()).collect()
    }

    /// Simulates preloading all registered models.
    pub fn preload_all(&mut self) -> Vec<PreloadRecord> {
        let ids: Vec<String> = self.models.keys().cloned().collect();
        let mut records = Vec::new();
        for id in ids {
            let size_mb = self.models[&id].size_mb();
            self.states.insert(id.clone(), PreloadState::Loaded);
            records.push(PreloadRecord::success(
                &id,
                Duration::from_millis(10),
                size_mb,
            ));
        }
        records
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates that a model file's checksum matches the descriptor.
pub fn validate_checksum(desc: &ModelDescriptor, actual_sha256: &str) -> bool {
    match &desc.sha256 {
        Some(expected) => expected == actual_sha256,
        None => true, // no checksum configured, skip validation
    }
}

/// Returns the total memory required to hold all listed models.
pub fn total_model_memory_mb(records: &[PreloadRecord]) -> u64 {
    records
        .iter()
        .filter(|r| r.state == PreloadState::Loaded)
        .map(|r| r.memory_mb)
        .sum()
}
