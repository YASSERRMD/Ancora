use ancora_controlplane::store::ControlPlaneStore;
use tracing::{info, warn};

/// Explicit release after successful step execution.
pub fn release_success(store: &mut ControlPlaneStore, worker_id: &str, run_id: &str) {
    info!(run_id, worker_id, "releasing lease on completion");
    store.release_lease(worker_id, run_id, true);
}

/// Explicit release after a step failure; run is requeued.
pub fn release_failure(store: &mut ControlPlaneStore, worker_id: &str, run_id: &str) {
    warn!(run_id, worker_id, "step failed, releasing and requeueing");
    store.release_lease(worker_id, run_id, false);
}
