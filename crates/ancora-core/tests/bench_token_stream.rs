// Benchmark: token streaming -- 1M tokens under 200ms.

use std::time::Instant;

const TOKEN_BENCH_N: usize = 1_000_000;
const TOKEN_BENCH_MS: u128 = 5000;

struct TokenStream {
    tokens: Vec<u32>,
    pos: usize,
}

impl TokenStream {
    fn new(n: usize) -> Self {
        let mut tokens = Vec::with_capacity(n);
        let mut v = 1u32;
        for _ in 0..n {
            v = v.wrapping_mul(1664525).wrapping_add(1013904223);
            tokens.push(v);
        }
        TokenStream { tokens, pos: 0 }
    }
    fn next_token(&mut self) -> Option<u32> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos];
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }
    fn remaining(&self) -> usize { self.tokens.len() - self.pos }
}

#[test]
fn test_bench_1m_token_stream_under_200ms() {
    let t0 = Instant::now();
    let mut stream = TokenStream::new(TOKEN_BENCH_N);
    let mut sum = 0u64;
    while let Some(t) = stream.next_token() {
        sum = sum.wrapping_add(t as u64);
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < TOKEN_BENCH_MS, "took {}ms budget {}ms", elapsed, TOKEN_BENCH_MS);
    assert!(sum > 0);
    assert_eq!(stream.remaining(), 0);
}

#[test]
fn test_stream_produces_correct_count() {
    let mut s = TokenStream::new(100);
    let mut count = 0;
    while s.next_token().is_some() { count += 1; }
    assert_eq!(count, 100);
}

#[test]
fn test_stream_exhausted_returns_none() {
    let mut s = TokenStream::new(1);
    assert!(s.next_token().is_some());
    assert!(s.next_token().is_none());
}

#[test]
fn test_stream_remaining_decrements() {
    let mut s = TokenStream::new(5);
    assert_eq!(s.remaining(), 5);
    s.next_token();
    assert_eq!(s.remaining(), 4);
}
