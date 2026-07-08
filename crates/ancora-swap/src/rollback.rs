/// Rollback helpers: manage a stack of prior model versions.
///
/// `RollbackStack` keeps a bounded history of model handles so that multiple
/// successive rollbacks can be performed (not just a single undo).
use std::collections::VecDeque;

use crate::model::ModelHandle;

/// A stack of model handles available for rollback.
pub struct RollbackStack {
    capacity: usize,
    stack: VecDeque<ModelHandle>,
}

impl RollbackStack {
    /// Create a new stack with the given capacity.  Once full, the oldest
    /// entry is dropped to make room for a new one.
    pub fn new(capacity: usize) -> Self {
        RollbackStack {
            capacity: capacity.max(1),
            stack: VecDeque::new(),
        }
    }

    /// Push a model onto the stack (called when it becomes active).
    pub fn push(&mut self, handle: ModelHandle) {
        if self.stack.len() == self.capacity {
            self.stack.pop_front();
        }
        self.stack.push_back(handle);
    }

    /// Pop the most-recently-pushed handle for rollback.
    pub fn pop(&mut self) -> Option<ModelHandle> {
        self.stack.pop_back()
    }

    /// Number of entries currently in the stack.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// True if no entries are stored.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
