// Load: journal throughput -- N events processed under time budget.

use std::time::Instant;

const LOAD_N: usize = 10_000;
const BUDGET_MS: u128 = 2_000;

struct JournalEntry {
    seq: u64,
    kind: &'static str,
    payload_bytes: usize,
}

fn generate_load_journal(n: usize) -> Vec<JournalEntry> {
    (0..n).map(|i| JournalEntry {
        seq: i as u64,
        kind: if i % 100 == 0 { "activity" } else { "token" },
        payload_bytes: 128,
    }).collect()
}

fn process_journal(entries: &[JournalEntry]) -> (usize, u64) {
    let mut activities = 0usize;
    let mut total_bytes = 0u64;
    for e in entries {
        if e.kind == "activity" { activities += 1; }
        total_bytes += e.payload_bytes as u64;
    }
    (activities, total_bytes)
}

#[test]
fn test_load_10k_events_within_budget() {
    let journal = generate_load_journal(LOAD_N);
    let t0 = Instant::now();
    let (acts, bytes) = process_journal(&journal);
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < BUDGET_MS, "took {}ms, budget {}ms", elapsed, BUDGET_MS);
    assert_eq!(acts, LOAD_N / 100);
    assert_eq!(bytes, (LOAD_N * 128) as u64);
}

#[test]
fn test_load_journal_has_correct_count() {
    let j = generate_load_journal(LOAD_N);
    assert_eq!(j.len(), LOAD_N);
}

#[test]
fn test_seq_is_monotonically_increasing() {
    let j = generate_load_journal(50);
    for (i, e) in j.iter().enumerate() { assert_eq!(e.seq, i as u64); }
}

#[test]
fn test_activity_events_every_100() {
    let j = generate_load_journal(500);
    let activity_seqs: Vec<u64> = j.iter().filter(|e| e.kind == "activity").map(|e| e.seq).collect();
    assert_eq!(activity_seqs, (0..500u64).step_by(100).collect::<Vec<_>>());
}

#[test]
fn test_total_bytes_correct() {
    let j = generate_load_journal(1000);
    let (_, bytes) = process_journal(&j);
    assert_eq!(bytes, 1000 * 128);
}
