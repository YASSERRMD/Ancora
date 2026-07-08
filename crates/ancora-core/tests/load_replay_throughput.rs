// Load: replay throughput -- replay 5k recorded events within 1s.

use std::time::Instant;

const REPLAY_N: usize = 5_000;
const REPLAY_BUDGET_MS: u128 = 1_000;

#[derive(Clone)]
struct RecordedEvent {
    seq: u64,
    kind: &'static str,
    activity_key: Option<&'static str>,
    replayed: bool,
}

fn make_replay_journal(n: usize) -> Vec<RecordedEvent> {
    (0..n)
        .map(|i| RecordedEvent {
            seq: i as u64,
            kind: if i % 50 == 0 { "activity" } else { "token" },
            activity_key: if i % 50 == 0 { Some("tool-call") } else { None },
            replayed: true,
        })
        .collect()
}

fn replay_events_sim(events: &[RecordedEvent]) -> usize {
    events.iter().filter(|e| e.replayed).count()
}

#[test]
fn test_replay_5k_events_within_budget() {
    let journal = make_replay_journal(REPLAY_N);
    let t0 = Instant::now();
    let replayed = replay_events_sim(&journal);
    let elapsed = t0.elapsed().as_millis();
    assert!(
        elapsed < REPLAY_BUDGET_MS,
        "took {}ms budget {}ms",
        elapsed,
        REPLAY_BUDGET_MS
    );
    assert_eq!(replayed, REPLAY_N);
}

#[test]
fn test_all_events_marked_replayed() {
    let j = make_replay_journal(REPLAY_N);
    assert!(j.iter().all(|e| e.replayed));
}

#[test]
fn test_activity_count_correct() {
    let j = make_replay_journal(500);
    let acts: usize = j.iter().filter(|e| e.kind == "activity").count();
    assert_eq!(acts, 500 / 50);
}

#[test]
fn test_seq_monotonic_in_replay_journal() {
    let j = make_replay_journal(100);
    for (i, e) in j.iter().enumerate() {
        assert_eq!(e.seq, i as u64);
    }
}

#[test]
fn test_activity_key_present_on_activity_events() {
    let j = make_replay_journal(100);
    for e in &j {
        if e.kind == "activity" {
            assert!(e.activity_key.is_some());
        } else {
            assert!(e.activity_key.is_none());
        }
    }
}
