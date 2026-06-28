use crate::pool::WorkerPool;
use crate::shutdown::ShutdownSignal;

/// Drive the pool in a simple run-to-completion loop.
pub fn run_to_idle(pool: &mut WorkerPool, max_ticks: usize) -> usize {
    let mut completed = 0;
    for _ in 0..max_ticks {
        if pool.is_idle() {
            break;
        }
        let results = pool.tick();
        for r in results {
            if let Ok(Some(_)) = r {
                completed += 1;
            }
        }
        pool.expire_leases();
    }
    completed
}

/// Drive the pool until the shutdown signal fires, draining in-flight runs.
pub fn run_with_shutdown(pool: &mut WorkerPool, signal: ShutdownSignal, max_ticks: usize) -> usize {
    let mut completed = 0;
    for _ in 0..max_ticks {
        if signal.is_requested() {
            pool.start_drain();
        }
        let results = pool.tick();
        for r in results {
            if let Ok(Some(_)) = r {
                completed += 1;
            }
        }
        pool.expire_leases();
        if signal.is_requested() && pool.is_idle() {
            break;
        }
    }
    completed
}
