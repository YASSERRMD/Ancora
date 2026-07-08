/// Memory reclaim: track and release unloaded model handles.
use crate::model::ModelHandle;

/// A reclaim queue holds handles that have been unloaded and are waiting for
/// their pin count to reach zero before memory can be freed.
#[derive(Default)]
pub struct ReclaimQueue {
    pending: Vec<ModelHandle>,
}

impl ReclaimQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        ReclaimQueue::default()
    }

    /// Add a handle to the reclaim queue.  The handle must already be unloaded.
    pub fn enqueue(&mut self, handle: ModelHandle) {
        self.pending.push(handle);
    }

    /// Sweep the queue: drop all handles that can be reclaimed (pin count == 0).
    ///
    /// Returns the number of handles reclaimed.
    pub fn sweep(&mut self) -> usize {
        let before = self.pending.len();
        self.pending.retain(|h| !h.can_reclaim());
        before - self.pending.len()
    }

    /// Number of handles waiting to be reclaimed.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Memory bytes still held by pending handles.
    pub fn pending_bytes(&self) -> u64 {
        self.pending.iter().map(|h| h.meta().memory_bytes).sum()
    }
}
