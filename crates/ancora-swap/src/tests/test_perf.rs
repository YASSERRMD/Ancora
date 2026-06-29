use std::time::Instant;
use crate::runtime::{make_model, SwapRuntime};

#[test]
fn test_swap_latency_measured() {
    let m1 = make_model("perf-base");
    let rt = SwapRuntime::new(m1);

    let m2 = make_model("perf-new");
    let before = Instant::now();
    let result = rt.swap(m2);
    let wall_ns = before.elapsed().as_nanos() as u64;

    assert!(result.elapsed_ns > 0, "latency must be non-zero");
    // The measured latency should be in the same ballpark as wall time.
    assert!(
        result.elapsed_ns <= wall_ns + 1_000_000,
        "measured latency must not exceed wall time by more than 1 ms"
    );

    let latencies = rt.swap_latencies_ns();
    assert_eq!(latencies.len(), 1);
    assert_eq!(latencies[0], result.elapsed_ns);
}
