use crate::store::ControlPlaneStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub live: bool,
    pub ready: bool,
    pub queue_depth: usize,
    pub worker_count: usize,
}

pub struct HealthApi<'a> {
    store: &'a ControlPlaneStore,
}

impl<'a> HealthApi<'a> {
    pub fn new(store: &'a ControlPlaneStore) -> Self {
        HealthApi { store }
    }

    pub fn liveness(&self) -> HealthStatus {
        HealthStatus {
            live: true,
            ready: true,
            queue_depth: self.store.queue_depth(),
            worker_count: self.store.worker_count(),
        }
    }

    pub fn readiness(&self) -> HealthStatus {
        // Ready when we have at least one worker or no queued runs
        let workers = self.store.worker_count();
        let depth = self.store.queue_depth();
        HealthStatus {
            live: true,
            ready: workers > 0 || depth == 0,
            queue_depth: depth,
            worker_count: workers,
        }
    }
}
