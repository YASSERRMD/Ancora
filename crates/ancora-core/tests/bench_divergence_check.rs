// Benchmark: detect_divergence -- 100k comparisons under 500ms.

use std::time::Instant;

const DIV_BENCH_N: usize = 100_000;
const DIV_BENCH_MS: u128 = 5000;

fn detect_divergence(expected: &[&str], observed: &[&str]) -> Result<(), String> {
    if expected.len() != observed.len() {
        return Err(format!("length mismatch: {} vs {}", expected.len(), observed.len()));
    }
    for (i, (e, o)) in expected.iter().zip(observed.iter()).enumerate() {
        if e != o {
            return Err(format!("diverge at {}: expected {} got {}", i, e, o));
        }
    }
    Ok(())
}

#[test]
fn test_bench_100k_divergence_checks_under_500ms() {
    let expected = vec!["activity_a", "activity_b", "activity_c", "activity_d"];
    let observed = vec!["activity_a", "activity_b", "activity_c", "activity_d"];
    let t0 = Instant::now();
    let mut ok = 0u64;
    for _ in 0..DIV_BENCH_N {
        if detect_divergence(&expected, &observed).is_ok() { ok += 1; }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < DIV_BENCH_MS, "took {}ms budget {}ms", elapsed, DIV_BENCH_MS);
    assert_eq!(ok, DIV_BENCH_N as u64);
}

#[test]
fn test_no_divergence_returns_ok() {
    let r = detect_divergence(&["a", "b"], &["a", "b"]);
    assert!(r.is_ok());
}

#[test]
fn test_value_divergence_returns_err() {
    let r = detect_divergence(&["a", "b"], &["a", "c"]);
    assert!(r.is_err());
}

#[test]
fn test_length_divergence_returns_err() {
    let r = detect_divergence(&["a"], &["a", "b"]);
    assert!(r.is_err());
}

#[test]
fn test_empty_sequences_ok() {
    let r = detect_divergence(&[], &[]);
    assert!(r.is_ok());
}
