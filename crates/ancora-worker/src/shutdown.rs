use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Graceful-shutdown signal shared between the pool and control thread.
#[derive(Clone, Default)]
pub struct ShutdownSignal {
    inner: Arc<AtomicBool>,
}

impl ShutdownSignal {
    pub fn new() -> Self {
        ShutdownSignal {
            inner: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn request(&self) {
        self.inner.store(true, Ordering::SeqCst);
    }

    pub fn is_requested(&self) -> bool {
        self.inner.load(Ordering::SeqCst)
    }
}
