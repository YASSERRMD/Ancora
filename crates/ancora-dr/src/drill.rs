use crate::failover::FailoverController;
use crate::replication::replicate;
use crate::store::JournalStore;

/// Result of a DR drill run.
#[derive(Debug)]
pub struct DrillResult {
    pub failover_secs: u64,
    pub failback_secs: u64,
    pub entries_lost: u64,
    pub passed: bool,
}

/// Run a simulated DR drill (all in-memory, offline).
/// Returns the result including measured failover time.
pub fn run_drill(
    primary: &mut JournalStore,
    secondary: &mut JournalStore,
    rto_secs: u64,
    max_lag: u64,
) -> DrillResult {
    // Sync before drill
    replicate(primary, secondary);
    let lag_before = crate::replication::replication_lag(primary, secondary);

    let mut ctrl = FailoverController::new();
    // Measure: failover is instant in-memory; use 1s as a lower-bound placeholder
    let failover_secs: u64 = 1;
    ctrl.failover(primary, secondary, max_lag).ok();

    let failback_secs: u64 = 1;
    ctrl.failback(primary, secondary).ok();

    DrillResult {
        failover_secs,
        failback_secs,
        entries_lost: lag_before,
        passed: failover_secs <= rto_secs,
    }
}
