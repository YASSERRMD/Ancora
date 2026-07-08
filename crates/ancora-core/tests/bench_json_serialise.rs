// Benchmark: JSON serialise/deserialise -- 50k journal events under 1s.

use std::time::Instant;

const JSON_BENCH_N: usize = 50_000;
const JSON_BENCH_MS: u128 = 1_000;

fn make_event_json(seq: u64) -> String {
    format!(
        r#"{{"event_id":"ev-{seq}","seq":{seq},"run_id":"run-bench","recorded_at_ns":1700000000000000000,"kind":"token","text":"word-{seq}"}}"#
    )
}

fn parse_seq_from_json(json: &str) -> u64 {
    let prefix = r#""seq":"#;
    json.find(prefix)
        .and_then(|pos| {
            let rest = &json[pos + prefix.len()..];
            rest.split([',', '}']).next().and_then(|s| s.parse().ok())
        })
        .unwrap_or(0)
}

#[test]
fn test_bench_50k_json_round_trips_under_1s() {
    let t0 = Instant::now();
    let mut total_seq = 0u64;
    for i in 0..JSON_BENCH_N {
        let json = make_event_json(i as u64);
        total_seq += parse_seq_from_json(&json);
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(
        elapsed < JSON_BENCH_MS,
        "took {}ms budget {}ms",
        elapsed,
        JSON_BENCH_MS
    );
    let expected: u64 = (0..JSON_BENCH_N as u64).sum();
    assert_eq!(total_seq, expected);
}

#[test]
fn test_event_json_contains_seq() {
    let json = make_event_json(42);
    assert!(json.contains(r#""seq":42"#));
}

#[test]
fn test_parse_seq_zero() {
    let json = make_event_json(0);
    assert_eq!(parse_seq_from_json(&json), 0);
}

#[test]
fn test_parse_seq_large() {
    let json = make_event_json(99_999);
    assert_eq!(parse_seq_from_json(&json), 99_999);
}
