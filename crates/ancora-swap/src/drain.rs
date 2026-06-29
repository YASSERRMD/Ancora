/// Graceful drain helpers.
///
/// When a model is swapped out it enters the drain state.  Existing runs that
/// hold a pin on the old model are allowed to complete normally.  Only after
/// all such runs finish does the model become eligible for memory reclaim.

use crate::model::ModelHandle;

/// Status of the drain process for a single model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrainStatus {
    /// The model is draining -- at least one run still holds a pin.
    Draining { remaining_pins: usize },
    /// No pins remaining; the model can be reclaimed.
    Complete,
    /// Not in drain state (model is still active or already reclaimed).
    Idle,
}

/// Inspect the drain status of a model handle.
pub fn drain_status(handle: &ModelHandle) -> DrainStatus {
    if !handle.is_unloaded() {
        return DrainStatus::Idle;
    }
    let pins = handle.pin_count();
    if pins > 0 {
        DrainStatus::Draining { remaining_pins: pins }
    } else {
        DrainStatus::Complete
    }
}

/// Poll until the given model handle has fully drained (all pins released).
/// In a real system this would use async / condvar; here we busy-loop with
/// a configurable sleep for use in tests.
///
/// Returns the number of poll iterations performed.
pub fn busy_drain(handle: &ModelHandle, poll_interval_ms: u64) -> usize {
    let mut iters = 0;
    loop {
        iters += 1;
        match drain_status(handle) {
            DrainStatus::Complete => return iters,
            DrainStatus::Idle => return iters, // not in drain state
            DrainStatus::Draining { .. } => {
                if poll_interval_ms > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(poll_interval_ms));
                }
            }
        }
    }
}
