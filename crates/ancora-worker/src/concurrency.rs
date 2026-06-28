use ancora_controlplane::store::ControlPlaneStore;

/// Check if a worker has capacity for another run.
pub fn worker_has_capacity(store: &ControlPlaneStore, worker_id: &str) -> bool {
    store
        .workers
        .get(worker_id)
        .map(|w| w.active_count < w.concurrency_limit)
        .unwrap_or(false)
}

/// Count total capacity across all workers.
pub fn total_capacity(store: &ControlPlaneStore) -> usize {
    store
        .workers
        .values()
        .map(|w| w.concurrency_limit.saturating_sub(w.active_count))
        .sum()
}
