/// Model load and unload management.
///
/// Tracks which models are currently loaded into memory and manages the
/// lifecycle of model handles.
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::registry::ModelEntry;

/// Unique handle ID for a loaded model instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadHandle(u64);

impl LoadHandle {
    fn next() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        LoadHandle(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl std::fmt::Display for LoadHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handle#{}", self.0)
    }
}

/// Reasons a model load can fail.
#[derive(Debug)]
pub enum LoadError {
    /// Model ID not found in registry.
    NotFound(String),
    /// Not enough memory available.
    OutOfMemory { needed: u64, available: u64 },
    /// Model file not found on disk.
    FileNotFound(String),
    /// Another error.
    Other(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::NotFound(id) => write!(f, "model not found: {}", id),
            LoadError::OutOfMemory { needed, available } => {
                write!(
                    f,
                    "out of memory: need {} bytes, have {}",
                    needed, available
                )
            }
            LoadError::FileNotFound(p) => write!(f, "file not found: {}", p),
            LoadError::Other(msg) => write!(f, "load error: {}", msg),
        }
    }
}

/// Reasons an unload can fail.
#[derive(Debug)]
pub enum UnloadError {
    /// Handle is not currently loaded.
    NotLoaded(LoadHandle),
}

impl std::fmt::Display for UnloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnloadError::NotLoaded(h) => write!(f, "model {} is not loaded", h),
        }
    }
}

/// Metadata about a loaded model instance.
#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub handle: LoadHandle,
    pub model_id: String,
    pub ram_bytes: u64,
    pub loaded_at: Instant,
}

impl LoadedModel {
    /// How long this model has been loaded.
    pub fn age(&self) -> Duration {
        self.loaded_at.elapsed()
    }
}

/// Runtime manager: tracks loaded models and available memory.
pub struct RuntimeManager {
    /// Total available RAM in bytes.
    total_ram: u64,
    /// Currently loaded models indexed by handle.
    loaded: HashMap<LoadHandle, LoadedModel>,
    /// Map from model ID to set of handles (supports multi-instance).
    id_to_handles: HashMap<String, HashSet<LoadHandle>>,
}

impl RuntimeManager {
    /// Create a new RuntimeManager with the given total RAM budget.
    pub fn new(total_ram_bytes: u64) -> Self {
        RuntimeManager {
            total_ram: total_ram_bytes,
            loaded: HashMap::new(),
            id_to_handles: HashMap::new(),
        }
    }

    /// Currently used RAM in bytes.
    pub fn used_ram(&self) -> u64 {
        self.loaded.values().map(|m| m.ram_bytes).sum()
    }

    /// Available RAM in bytes.
    pub fn available_ram(&self) -> u64 {
        self.total_ram.saturating_sub(self.used_ram())
    }

    /// Total RAM budget in bytes.
    pub fn total_ram(&self) -> u64 {
        self.total_ram
    }

    /// Load a model into memory (simulation -- no actual file I/O).
    ///
    /// Returns a `LoadHandle` on success.
    pub fn load(&mut self, model_id: &str, entry: &ModelEntry) -> Result<LoadHandle, LoadError> {
        let ram_needed = entry.estimated_ram_bytes();
        let available = self.available_ram();

        if ram_needed > available {
            return Err(LoadError::OutOfMemory {
                needed: ram_needed,
                available,
            });
        }

        let handle = LoadHandle::next();
        let loaded = LoadedModel {
            handle,
            model_id: model_id.to_string(),
            ram_bytes: ram_needed,
            loaded_at: Instant::now(),
        };
        self.loaded.insert(handle, loaded);
        self.id_to_handles
            .entry(model_id.to_string())
            .or_default()
            .insert(handle);

        Ok(handle)
    }

    /// Unload a model by handle.
    pub fn unload(&mut self, handle: LoadHandle) -> Result<LoadedModel, UnloadError> {
        let model = self
            .loaded
            .remove(&handle)
            .ok_or(UnloadError::NotLoaded(handle))?;
        if let Some(handles) = self.id_to_handles.get_mut(&model.model_id) {
            handles.remove(&handle);
        }
        Ok(model)
    }

    /// Unload all instances of a given model ID.
    pub fn unload_all_by_id(&mut self, model_id: &str) -> usize {
        let handles: Vec<LoadHandle> = self
            .id_to_handles
            .get(model_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect();
        let count = handles.len();
        for h in handles {
            self.loaded.remove(&h);
        }
        self.id_to_handles.remove(model_id);
        count
    }

    /// Check if a model ID is currently loaded (at least one handle).
    pub fn is_loaded(&self, model_id: &str) -> bool {
        self.id_to_handles
            .get(model_id)
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    }

    /// Get all loaded model metadata.
    pub fn loaded_models(&self) -> impl Iterator<Item = &LoadedModel> {
        self.loaded.values()
    }

    /// Number of currently loaded models (handles).
    pub fn loaded_count(&self) -> usize {
        self.loaded.len()
    }

    /// Evict the least recently used model to free memory.
    /// Returns the unloaded model or None if nothing is loaded.
    pub fn evict_lru(&mut self) -> Option<LoadedModel> {
        let oldest_handle = self
            .loaded
            .values()
            .max_by_key(|m| m.loaded_at.elapsed())
            .map(|m| m.handle)?;
        self.unload(oldest_handle).ok()
    }
}
