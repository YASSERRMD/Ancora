// Chaos: clock skew -- replay tolerates out-of-order wall-clock timestamps.

struct Event {
    seq: u64,
    #[allow(dead_code)]
    recorded_at_ns: u64,
    kind: &'static str,
}

fn validate_seq_order(events: &[Event]) -> bool {
    events.windows(2).all(|w| w[0].seq < w[1].seq)
}

fn validate_timestamp_not_required_monotonic(events: &[Event]) -> bool {
    // timestamps may decrease due to clock skew -- that is acceptable
    let _ = events;
    true
}

fn replay_in_seq_order(events: &mut [Event]) -> Vec<&str> {
    events.sort_by_key(|e| e.seq);
    events.iter().map(|e| e.kind).collect()
}

fn events_with_skew() -> Vec<Event> {
    vec![
        Event {
            seq: 0,
            recorded_at_ns: 1_000_000_000,
            kind: "started",
        },
        Event {
            seq: 1,
            recorded_at_ns: 999_000_000,
            kind: "activity",
        }, // skewed back
        Event {
            seq: 2,
            recorded_at_ns: 1_100_000_000,
            kind: "completed",
        },
    ]
}

#[test]
fn test_skewed_timestamps_do_not_break_replay() {
    let mut evs = events_with_skew();
    let kinds = replay_in_seq_order(&mut evs);
    assert_eq!(kinds, vec!["started", "activity", "completed"]);
}

#[test]
fn test_seq_order_is_valid_after_sort() {
    let mut evs = events_with_skew();
    evs.sort_by_key(|e| e.seq);
    assert!(validate_seq_order(&evs));
}

#[test]
fn test_timestamp_skew_is_accepted() {
    let evs = events_with_skew();
    assert!(validate_timestamp_not_required_monotonic(&evs));
}

#[test]
fn test_large_negative_skew_still_replays_correctly() {
    let mut evs = vec![
        Event {
            seq: 0,
            recorded_at_ns: 9_000_000_000,
            kind: "started",
        },
        Event {
            seq: 1,
            recorded_at_ns: 1,
            kind: "activity",
        },
        Event {
            seq: 2,
            recorded_at_ns: 9_000_000_001,
            kind: "completed",
        },
    ];
    let kinds = replay_in_seq_order(&mut evs);
    assert_eq!(kinds[0], "started");
    assert_eq!(kinds[2], "completed");
}

#[test]
fn test_equal_timestamps_different_seqs_ordered_by_seq() {
    let mut evs = vec![
        Event {
            seq: 2,
            recorded_at_ns: 100,
            kind: "b",
        },
        Event {
            seq: 1,
            recorded_at_ns: 100,
            kind: "a",
        },
    ];
    let kinds = replay_in_seq_order(&mut evs);
    assert_eq!(kinds, vec!["a", "b"]);
}

#[test]
fn test_single_event_replay_is_valid() {
    let mut evs = vec![Event {
        seq: 0,
        recorded_at_ns: 42,
        kind: "started",
    }];
    let kinds = replay_in_seq_order(&mut evs);
    assert_eq!(kinds, vec!["started"]);
}
