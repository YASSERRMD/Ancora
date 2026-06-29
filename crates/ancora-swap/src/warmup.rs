/// Warmup gate: ensure a model is ready before it is swapped in.
///
/// A `WarmupGate` wraps a candidate `ModelHandle` and records whether
/// warmup has been completed.  The swap runtime should refuse to swap in
/// any handle whose gate is not cleared.

use std::time::{Duration, Instant};

use crate::model::ModelHandle;
use crate::runtime::WarmupStatus;

/// A warmup gate for a candidate model.
pub struct WarmupGate {
    handle: ModelHandle,
    status: WarmupStatus,
}

impl WarmupGate {
    /// Create a gate in `InProgress` state.
    pub fn new(handle: ModelHandle) -> Self {
        WarmupGate {
            handle,
            status: WarmupStatus::InProgress,
        }
    }

    /// Run the warmup procedure (simulated).
    ///
    /// `synthetic_prompts` is the number of synthetic forward-passes to
    /// simulate; `cost_per_prompt_ms` is the simulated time each one takes.
    pub fn run(&mut self, synthetic_prompts: u32, cost_per_prompt_ms: u64) {
        let start = Instant::now();
        for _ in 0..synthetic_prompts {
            if cost_per_prompt_ms > 0 {
                std::thread::sleep(Duration::from_millis(cost_per_prompt_ms));
            }
        }
        self.status = WarmupStatus::Complete(start.elapsed());
    }

    /// Whether warmup has completed successfully.
    pub fn is_ready(&self) -> bool {
        matches!(self.status, WarmupStatus::Complete(_))
    }

    /// Current status.
    pub fn status(&self) -> &WarmupStatus {
        &self.status
    }

    /// Consume the gate and return the handle if warmup is complete.
    pub fn into_handle(self) -> Result<ModelHandle, &'static str> {
        if self.is_ready() {
            Ok(self.handle)
        } else {
            Err("warmup not complete")
        }
    }
}
