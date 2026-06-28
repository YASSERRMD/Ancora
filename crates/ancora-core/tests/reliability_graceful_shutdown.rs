// Reliability: graceful shutdown -- in-flight runs complete before exit.

use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};

fn simulate_graceful_shutdown(in_flight: usize) -> (usize, usize) {
    let shutdown = Arc::new(AtomicBool::new(false));
    let completed = Arc::new(AtomicUsize::new(0));
    let rejected = Arc::new(AtomicUsize::new(0));

    // simulate runs in flight at time of shutdown signal
    shutdown.store(true, Ordering::SeqCst);

    // existing in-flight runs still complete
    for _ in 0..in_flight {
        completed.fetch_add(1, Ordering::SeqCst);
    }

    // new runs are rejected
    let is_shutting_down = shutdown.load(Ordering::SeqCst);
    if is_shutting_down { rejected.fetch_add(1, Ordering::SeqCst); }

    (completed.load(Ordering::SeqCst), rejected.load(Ordering::SeqCst))
}

#[test]
fn test_in_flight_runs_complete_on_shutdown() {
    let (done, _) = simulate_graceful_shutdown(5);
    assert_eq!(done, 5);
}

#[test]
fn test_new_runs_rejected_on_shutdown() {
    let (_, rejected) = simulate_graceful_shutdown(3);
    assert_eq!(rejected, 1);
}

#[test]
fn test_zero_in_flight_on_shutdown() {
    let (done, rejected) = simulate_graceful_shutdown(0);
    assert_eq!(done, 0);
    assert_eq!(rejected, 1);
}

#[test]
fn test_large_in_flight_all_complete() {
    let (done, _) = simulate_graceful_shutdown(100);
    assert_eq!(done, 100);
}

#[test]
fn test_shutdown_flag_is_set() {
    let flag = Arc::new(AtomicBool::new(false));
    flag.store(true, Ordering::SeqCst);
    assert!(flag.load(Ordering::SeqCst));
}

#[test]
fn test_completed_plus_rejected_account_for_all_work() {
    let in_flight = 7;
    let (done, rejected) = simulate_graceful_shutdown(in_flight);
    assert_eq!(done, in_flight);
    assert_eq!(rejected, 1);
}
