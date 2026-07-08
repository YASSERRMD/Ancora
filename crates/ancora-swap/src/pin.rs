/// Version pinning API: lock a run to a specific model version for its lifetime.
///
/// The `PinRegistry` lets callers explicitly assign a version to a run before
/// the run starts, regardless of what the active model is at start time.
use std::collections::HashMap;
use std::sync::Mutex;

use crate::model::ModelHandle;
use crate::runtime::RunId;

/// Registry mapping run IDs to pinned model handles.
pub struct PinRegistry {
    inner: Mutex<HashMap<RunId, ModelHandle>>,
}

impl PinRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        PinRegistry {
            inner: Mutex::new(HashMap::new()),
        }
    }

    /// Pin `run` to `handle`.  Any existing pin for the run is replaced.
    pub fn pin_run(&self, run: RunId, handle: ModelHandle) {
        self.inner.lock().unwrap().insert(run, handle);
    }

    /// Remove and return the pinned handle for `run`, if any.
    pub fn unpin_run(&self, run: RunId) -> Option<ModelHandle> {
        self.inner.lock().unwrap().remove(&run)
    }

    /// Look up the pinned model for a run without removing it.
    pub fn get(&self, run: RunId) -> Option<ModelHandle> {
        self.inner.lock().unwrap().get(&run).cloned()
    }

    /// Number of runs currently pinned.
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }

    /// True if no runs are pinned.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for PinRegistry {
    fn default() -> Self {
        Self::new()
    }
}
