// Benchmark: replay speed -- 10k events replayed under 500ms.

use std::time::Instant;

const REPLAY_BENCH_N: usize = 10_000;
const REPLAY_BENCH_MS: u128 = 500;

fn make_replay_batch(n: usize) -> Vec<(u64, bool)> {
    (0..n).map(|i| (i as u64, i % 10 == 0)).collect()
}

fn replay_batch(entries: &[(u64, bool)]) -> usize {
    entries.iter().filter(|(_, is_activity)| *is_activity).count()
}

#[test]
fn test_bench_10k_replay_under_500ms() {
    let t0 = Instant::now();
    let batch = make_replay_batch(REPLAY_BENCH_N);
    let acts = replay_batch(&batch);
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < REPLAY_BENCH_MS, "took {}ms budget {}ms", elapsed, REPLAY_BENCH_MS);
    assert_eq!(acts, REPLAY_BENCH_N / 10);
}

#[test]
fn test_bench_replay_activity_count() {
    let batch = make_replay_batch(100);
    assert_eq!(replay_batch(&batch), 10);
}

#[test]
fn test_bench_batch_len() {
    let batch = make_replay_batch(REPLAY_BENCH_N);
    assert_eq!(batch.len(), REPLAY_BENCH_N);
}

#[test]
fn test_bench_seq_zero_is_activity() {
    let batch = make_replay_batch(1);
    assert!(batch[0].1);
}
