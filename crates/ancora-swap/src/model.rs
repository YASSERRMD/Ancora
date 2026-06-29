/// Model handle abstraction for hot-swapping.
///
/// A `ModelHandle` represents a loaded model that can be referenced by
/// in-flight runs. Handles are reference-counted so memory is only reclaimed
/// once all runs that pinned a particular version have finished.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

/// Unique, monotonically-increasing model version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub struct ModelVersion(pub u64);

impl ModelVersion {
    /// Return the next version number using a global counter.
    pub fn next() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        ModelVersion(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl std::fmt::Display for ModelVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// Metadata associated with a particular model version.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelMeta {
    /// Human-readable name of the model (e.g. "gpt-4o-mini-2025-01").
    pub name: String,
    /// Semantic version string for the weights / checkpoint.
    pub version: String,
    /// Bytes occupied by the model weights in memory.
    pub memory_bytes: u64,
}

/// Inner state shared by all clones of a `ModelHandle`.
struct Inner {
    meta: ModelMeta,
    version: ModelVersion,
    /// How many active pins exist (i.e. runs that are using this model).
    pin_count: AtomicUsize,
    /// Whether this handle has been unloaded.
    unloaded: std::sync::Mutex<bool>,
}

/// A cloneable, reference-counted handle to a loaded model.
#[derive(Clone)]
pub struct ModelHandle(Arc<Inner>);

impl std::fmt::Debug for ModelHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelHandle")
            .field("name", &self.0.meta.name)
            .field("version", &self.0.version)
            .finish()
    }
}

impl ModelHandle {
    /// Create a new handle for the given model metadata.
    pub fn new(meta: ModelMeta, version: ModelVersion) -> Self {
        ModelHandle(Arc::new(Inner {
            meta,
            version,
            pin_count: AtomicUsize::new(0),
            unloaded: std::sync::Mutex::new(false),
        }))
    }

    /// The monotonic version identifier.
    pub fn version(&self) -> ModelVersion {
        self.0.version
    }

    /// Metadata about the loaded model.
    pub fn meta(&self) -> &ModelMeta {
        &self.0.meta
    }

    /// Increment the pin count and return a `Pin` guard that decrements on drop.
    /// Returns `None` if the model has already been unloaded.
    pub fn pin(&self) -> Option<ModelPin> {
        let unloaded = self.0.unloaded.lock().unwrap();
        if *unloaded {
            return None;
        }
        self.0.pin_count.fetch_add(1, Ordering::SeqCst);
        Some(ModelPin { handle: self.clone() })
    }

    /// Current number of active pins.
    pub fn pin_count(&self) -> usize {
        self.0.pin_count.load(Ordering::SeqCst)
    }

    /// Mark the model as unloaded so no new pins can be created.
    /// Existing pins remain valid; memory is conceptually released once the
    /// pin count drops to zero.
    pub fn unload(&self) {
        let mut unloaded = self.0.unloaded.lock().unwrap();
        *unloaded = true;
    }

    /// Re-enable new pins on a model that was previously unloaded.
    /// Used during rollback to restore a drain candidate to active status.
    pub fn reload(&self) {
        let mut unloaded = self.0.unloaded.lock().unwrap();
        *unloaded = false;
    }

    /// Returns `true` if `unload` has been called.
    pub fn is_unloaded(&self) -> bool {
        *self.0.unloaded.lock().unwrap()
    }

    /// Returns `true` when the model has been unloaded and all pins released.
    pub fn can_reclaim(&self) -> bool {
        self.is_unloaded() && self.pin_count() == 0
    }
}

/// RAII guard that keeps a model pinned (preventing memory reclaim) until dropped.
pub struct ModelPin {
    handle: ModelHandle,
}

impl ModelPin {
    /// Access the underlying handle.
    pub fn handle(&self) -> &ModelHandle {
        &self.handle
    }
}

impl Drop for ModelPin {
    fn drop(&mut self) {
        self.handle.0.pin_count.fetch_sub(1, Ordering::SeqCst);
    }
}

impl std::fmt::Debug for ModelPin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModelPin({})", self.handle.version())
    }
}
