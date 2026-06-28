use ancora_controlplane::store::ControlPlaneStore;
use tracing::info;

/// Drive periodic lease expiry to requeue runs from crashed workers.
pub struct LeaseReaper;

impl LeaseReaper {
    pub fn reap(store: &mut ControlPlaneStore) -> usize {
        let before = store.queue_depth();
        store.expire_leases();
        let after = store.queue_depth();
        let requeued = after.saturating_sub(before);
        if requeued > 0 {
            info!(requeued, "requeued runs from expired leases");
        }
        requeued
    }
}
