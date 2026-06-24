use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Read-only token checked by the executor before each node to honour cancellation.
pub struct CancellationToken {
    flag: Arc<AtomicBool>,
}

/// Write-side handle used by the caller to signal cancellation.
pub struct CancellationHandle {
    flag: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Returns `true` once the paired `CancellationHandle::cancel` has been called.
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }
}

impl CancellationHandle {
    /// Signal cancellation to the paired `CancellationToken`.
    pub fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }
}

/// Create a linked `(CancellationToken, CancellationHandle)` pair.
pub fn cancellation_pair() -> (CancellationToken, CancellationHandle) {
    let flag = Arc::new(AtomicBool::new(false));
    (
        CancellationToken { flag: Arc::clone(&flag) },
        CancellationHandle { flag },
    )
}
