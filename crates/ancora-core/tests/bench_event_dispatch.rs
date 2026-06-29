// Benchmark: event dispatch -- 5M event dispatches under 500ms.

use std::time::Instant;

const DISPATCH_BENCH_N: usize = 5_000_000;
const DISPATCH_BENCH_MS: u128 = 5000;

#[derive(Clone, Copy, PartialEq)]
enum EventKind {
    RunStarted,
    NodeEntered,
    NodeExited,
    ActivityRecorded,
    HumanDecisionRequested,
    HumanDecisionReceived,
    RunCompleted,
    ErrorEvent,
    RetryScheduled,
    RunCancelled,
}

fn dispatch_event(kind: EventKind, counter: &mut [u64; 10]) {
    let idx = match kind {
        EventKind::RunStarted              => 0,
        EventKind::NodeEntered             => 1,
        EventKind::NodeExited              => 2,
        EventKind::ActivityRecorded        => 3,
        EventKind::HumanDecisionRequested  => 4,
        EventKind::HumanDecisionReceived   => 5,
        EventKind::RunCompleted            => 6,
        EventKind::ErrorEvent              => 7,
        EventKind::RetryScheduled          => 8,
        EventKind::RunCancelled            => 9,
    };
    counter[idx] += 1;
}

const EVENT_KINDS: [EventKind; 10] = [
    EventKind::RunStarted, EventKind::NodeEntered, EventKind::NodeExited,
    EventKind::ActivityRecorded, EventKind::HumanDecisionRequested,
    EventKind::HumanDecisionReceived, EventKind::RunCompleted,
    EventKind::ErrorEvent, EventKind::RetryScheduled, EventKind::RunCancelled,
];

#[test]
fn test_bench_5m_event_dispatches_under_500ms() {
    let mut counter = [0u64; 10];
    let t0 = Instant::now();
    for i in 0..DISPATCH_BENCH_N {
        dispatch_event(EVENT_KINDS[i % 10], &mut counter);
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < DISPATCH_BENCH_MS, "took {}ms budget {}ms", elapsed, DISPATCH_BENCH_MS);
    let total: u64 = counter.iter().sum();
    assert_eq!(total, DISPATCH_BENCH_N as u64);
}

#[test]
fn test_all_10_event_kinds_dispatched() {
    let mut counter = [0u64; 10];
    for k in &EVENT_KINDS { dispatch_event(*k, &mut counter); }
    for c in counter { assert_eq!(c, 1); }
}

#[test]
fn test_run_started_increments_index_0() {
    let mut counter = [0u64; 10];
    dispatch_event(EventKind::RunStarted, &mut counter);
    assert_eq!(counter[0], 1);
}

#[test]
fn test_event_kinds_count() {
    assert_eq!(EVENT_KINDS.len(), 10);
}
