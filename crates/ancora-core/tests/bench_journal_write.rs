// Benchmark: journal write throughput -- 100k entries under 2s.

use std::time::Instant;

const BENCH_N: usize = 100_000;
const BENCH_BUDGET_MS: u128 = 2_000;

struct BenchEntry {
    seq: u64,
    payload: [u8; 64],
}

fn write_bench_journal(n: usize) -> Vec<BenchEntry> {
    (0..n).map(|i| BenchEntry {
        seq: i as u64,
        payload: [(i & 0xFF) as u8; 64],
    }).collect()
}

fn read_bench_journal(entries: &[BenchEntry]) -> u64 {
    entries.iter().map(|e| e.seq).sum()
}

#[test]
fn test_bench_100k_journal_writes_under_2s() {
    let t0 = Instant::now();
    let entries = write_bench_journal(BENCH_N);
    let _ = read_bench_journal(&entries);
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < BENCH_BUDGET_MS, "took {}ms budget {}ms", elapsed, BENCH_BUDGET_MS);
    assert_eq!(entries.len(), BENCH_N);
}

#[test]
fn test_bench_entry_seq_correct() {
    let entries = write_bench_journal(10);
    for (i, e) in entries.iter().enumerate() { assert_eq!(e.seq, i as u64); }
}

#[test]
fn test_bench_payload_size_64_bytes() {
    let entries = write_bench_journal(1);
    assert_eq!(entries[0].payload.len(), 64);
}

#[test]
fn test_bench_sum_of_seqs() {
    let entries = write_bench_journal(5);
    let s = read_bench_journal(&entries);
    assert_eq!(s, 0 + 1 + 2 + 3 + 4);
}
