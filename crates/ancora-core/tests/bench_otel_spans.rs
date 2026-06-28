// Benchmark: OTel span construction -- 200k spans under 1s.

use std::time::Instant;

const OTEL_BENCH_N: usize = 200_000;
const OTEL_BENCH_MS: u128 = 1_000;

struct Span {
    trace_id: [u8; 16],
    span_id: [u8; 8],
    operation: u64,
    status: u8,
}

impl Span {
    fn new(idx: u64) -> Self {
        let mut trace_id = [0u8; 16];
        for i in 0..8 {
            trace_id[i] = ((idx >> (i * 4)) & 0xff) as u8;
        }
        Span {
            trace_id,
            span_id: [idx as u8; 8],
            operation: idx,
            status: (idx % 3) as u8,
        }
    }
    fn is_ok(&self) -> bool { self.status == 0 }
}

#[test]
fn test_bench_200k_otel_spans_under_1s() {
    let t0 = Instant::now();
    let mut ok_count = 0u64;
    for i in 0..OTEL_BENCH_N {
        let span = Span::new(i as u64);
        if span.is_ok() { ok_count += 1; }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < OTEL_BENCH_MS, "took {}ms budget {}ms", elapsed, OTEL_BENCH_MS);
    let expected = OTEL_BENCH_N as u64 / 3 + 1;
    assert!(ok_count <= expected);
}

#[test]
fn test_span_trace_id_length() {
    let span = Span::new(42);
    assert_eq!(span.trace_id.len(), 16);
}

#[test]
fn test_span_span_id_length() {
    let span = Span::new(0);
    assert_eq!(span.span_id.len(), 8);
}

#[test]
fn test_span_status_cycles() {
    let s0 = Span::new(0); assert!(s0.is_ok());
    let s1 = Span::new(1); assert!(!s1.is_ok());
    let s3 = Span::new(3); assert!(s3.is_ok());
}
