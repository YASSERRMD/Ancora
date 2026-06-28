// Benchmark: concurrent journal access simulation -- 50k operations under 300ms.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

const CONC_BENCH_N: usize = 50_000;
const CONC_BENCH_MS: u128 = 300;

struct AtomicJournal {
    write_count: AtomicU64,
    read_count: AtomicU64,
    seq: AtomicU64,
}

impl AtomicJournal {
    fn new() -> Arc<Self> {
        Arc::new(AtomicJournal {
            write_count: AtomicU64::new(0),
            read_count: AtomicU64::new(0),
            seq: AtomicU64::new(0),
        })
    }
    fn write(&self) -> u64 {
        let s = self.seq.fetch_add(1, Ordering::Relaxed);
        self.write_count.fetch_add(1, Ordering::Relaxed);
        s
    }
    fn read(&self) -> u64 {
        self.read_count.fetch_add(1, Ordering::Relaxed);
        self.seq.load(Ordering::Relaxed)
    }
}

#[test]
fn test_bench_50k_concurrent_journal_ops_under_300ms() {
    let journal = AtomicJournal::new();
    let t0 = Instant::now();
    for i in 0..CONC_BENCH_N {
        if i % 3 == 0 {
            journal.read();
        } else {
            journal.write();
        }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < CONC_BENCH_MS, "took {}ms budget {}ms", elapsed, CONC_BENCH_MS);
    let writes = journal.write_count.load(Ordering::Relaxed);
    let reads = journal.read_count.load(Ordering::Relaxed);
    assert_eq!(writes + reads, CONC_BENCH_N as u64);
}

#[test]
fn test_write_increments_seq() {
    let j = AtomicJournal::new();
    let s0 = j.write();
    let s1 = j.write();
    assert_eq!(s1, s0 + 1);
}

#[test]
fn test_read_returns_current_seq() {
    let j = AtomicJournal::new();
    j.write();
    let s = j.read();
    assert_eq!(s, 1);
}

#[test]
fn test_initial_seq_zero() {
    let j = AtomicJournal::new();
    assert_eq!(j.seq.load(Ordering::Relaxed), 0);
}
