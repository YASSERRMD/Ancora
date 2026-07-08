/// Runtime binding, hot-swap, graceful drain, warmup, and memory reclaim.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::journal::{JournalEntry, SwapEvent, SwapJournal};
use crate::model::{ModelHandle, ModelMeta, ModelPin, ModelVersion};

/// Identifies a run (in-flight inference request or agent turn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RunId(pub u64);

impl std::fmt::Display for RunId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "run-{}", self.0)
    }
}

/// State of a warm-up procedure for a new model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarmupStatus {
    /// Warmup is in progress.
    InProgress,
    /// Warmup completed successfully after the given duration.
    Complete(Duration),
    /// Warmup failed with an error message.
    Failed(String),
}

/// A runtime binding that tracks the active model and all in-flight runs.
pub struct SwapRuntime {
    inner: Arc<Mutex<RuntimeInner>>,
}

struct RuntimeInner {
    /// The currently active (swapped-in) model.
    active: ModelHandle,
    /// Per-run pinned model (for version pinning).
    run_pins: HashMap<RunId, ModelPin>,
    /// Swap journal for replay / audit.
    journal: SwapJournal,
    /// Previous model handle retained until drained.
    draining: Option<ModelHandle>,
    /// Measured swap latencies in nanoseconds.
    swap_latencies_ns: Vec<u64>,
}

impl SwapRuntime {
    /// Create a new runtime with the given initial model.
    pub fn new(initial: ModelHandle) -> Self {
        let journal = SwapJournal::new();
        SwapRuntime {
            inner: Arc::new(Mutex::new(RuntimeInner {
                active: initial,
                run_pins: HashMap::new(),
                journal,
                draining: None,
                swap_latencies_ns: Vec::new(),
            })),
        }
    }

    /// Return a clone of the currently-active model handle.
    pub fn active_model(&self) -> ModelHandle {
        self.inner.lock().unwrap().active.clone()
    }

    /// Start a new run: pin the current active model to the run so that even
    /// after a swap, the run continues to use its originally-assigned model.
    ///
    /// Returns `None` if the model has been unloaded (should not happen under
    /// normal operation).
    pub fn start_run(&self, id: RunId) -> Option<()> {
        let mut g = self.inner.lock().unwrap();
        let pin = g.active.pin()?;
        g.run_pins.insert(id, pin);
        Some(())
    }

    /// Finish a run and release its model pin.
    pub fn finish_run(&self, id: RunId) {
        let mut g = self.inner.lock().unwrap();
        g.run_pins.remove(&id);
        // If the draining model now has zero pins we can officially reclaim it.
        if let Some(draining) = &g.draining {
            if draining.can_reclaim() {
                g.draining = None;
            }
        }
    }

    /// Return the version pinned to a specific run, if any.
    pub fn run_model_version(&self, id: RunId) -> Option<ModelVersion> {
        let g = self.inner.lock().unwrap();
        g.run_pins.get(&id).map(|p| p.handle().version())
    }

    /// Hot-swap the active model.
    ///
    /// The new model must have already been warmed up (see `warmup`). The old
    /// model is moved to `draining` status — existing runs keep their pin and
    /// continue to work. New runs will receive a pin on `new_model`.
    ///
    /// The swap event is appended to the journal.
    pub fn swap(&self, new_model: ModelHandle) -> SwapResult {
        let start = Instant::now();
        let mut g = self.inner.lock().unwrap();

        let old = g.active.clone();
        let old_version = old.version();
        let new_version = new_model.version();

        // Mark old model so no new pins can be created on it.
        old.unload();

        // Keep a reference to old so in-flight runs can drain.
        g.draining = Some(old.clone());

        // Activate the new model.
        g.active = new_model;

        let elapsed_ns = start.elapsed().as_nanos() as u64;
        g.swap_latencies_ns.push(elapsed_ns);

        g.journal.append(JournalEntry {
            event: SwapEvent::Swap {
                from: old_version,
                to: new_version,
            },
            timestamp_ns: elapsed_ns,
        });

        SwapResult {
            old_version,
            new_version,
            elapsed_ns,
        }
    }

    /// Roll back to the previous model (the one in `draining` state).
    ///
    /// Fails if there is no previous model to roll back to, or if the previous
    /// model has already had its memory reclaimed (pin count 0 AND unloaded).
    pub fn rollback(&self) -> Result<RollbackResult, &'static str> {
        let mut g = self.inner.lock().unwrap();

        let prev = g
            .draining
            .take()
            .ok_or("no draining model to roll back to")?;

        // Un-unload the old model so new pins can be created again.
        prev.reload();

        let old_version = g.active.version();
        let prev_version = prev.version();

        // Current active becomes the drain candidate.
        let current = g.active.clone();
        current.unload();
        g.draining = Some(current);

        g.active = prev;

        g.journal.append(JournalEntry {
            event: SwapEvent::Rollback {
                from: old_version,
                to: prev_version,
            },
            timestamp_ns: 0,
        });

        Ok(RollbackResult {
            restored_version: prev_version,
        })
    }

    /// Perform a warmup of a candidate model before it is swapped in.
    ///
    /// In a real implementation this would pre-load the weights, run a
    /// synthetic prompt, etc. Here we simulate it with a configurable delay.
    pub fn warmup(&self, candidate: &ModelHandle, simulated_ms: u64) -> WarmupStatus {
        let start = Instant::now();
        // Simulate I/O / weight loading.
        if simulated_ms > 0 {
            std::thread::sleep(Duration::from_millis(simulated_ms));
        }
        // In a real system we might fail here if OOM or model file is corrupt.
        let _ = candidate; // suppress unused warning in simulation
        WarmupStatus::Complete(start.elapsed())
    }

    /// Reclaim memory for any unloaded models that have no remaining pins.
    ///
    /// Returns the number of models whose handles were released.
    pub fn reclaim_unloaded(&self) -> usize {
        let mut g = self.inner.lock().unwrap();
        let mut reclaimed = 0;
        if let Some(draining) = &g.draining {
            if draining.can_reclaim() {
                g.draining = None;
                reclaimed += 1;
            }
        }
        reclaimed
    }

    /// Number of runs currently in-flight.
    pub fn active_run_count(&self) -> usize {
        self.inner.lock().unwrap().run_pins.len()
    }

    /// Clone of the swap journal for inspection / replay.
    pub fn journal(&self) -> SwapJournal {
        self.inner.lock().unwrap().journal.clone()
    }

    /// All measured swap latencies in nanoseconds.
    pub fn swap_latencies_ns(&self) -> Vec<u64> {
        self.inner.lock().unwrap().swap_latencies_ns.clone()
    }

    /// Whether a drain model is currently outstanding.
    pub fn is_draining(&self) -> bool {
        self.inner.lock().unwrap().draining.is_some()
    }
}

/// Result of a successful hot-swap.
#[derive(Debug, Clone)]
pub struct SwapResult {
    pub old_version: ModelVersion,
    pub new_version: ModelVersion,
    pub elapsed_ns: u64,
}

/// Result of a successful rollback.
#[derive(Debug, Clone)]
pub struct RollbackResult {
    pub restored_version: ModelVersion,
}

/// Convenience builder for a model handle used in tests.
pub fn make_model(name: &str) -> ModelHandle {
    let version = ModelVersion::next();
    let meta = ModelMeta {
        name: name.to_string(),
        version: format!("0.1.{}", version.0),
        memory_bytes: 1024 * 1024 * 512, // 512 MiB
    };
    ModelHandle::new(meta, version)
}
