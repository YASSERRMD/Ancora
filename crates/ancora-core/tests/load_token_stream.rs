// Load: token stream -- high-volume streaming tokens stay within latency budget.

use std::time::Instant;

const TOKEN_COUNT: usize = 50_000;
const STREAM_BUDGET_MS: u128 = 1_000;

struct TokenStream {
    tokens: Vec<String>,
    cursor: usize,
}

impl TokenStream {
    fn new(n: usize) -> Self {
        Self { tokens: (0..n).map(|i| format!("tok{i}")).collect(), cursor: 0 }
    }
    fn next(&mut self) -> Option<&str> {
        if self.cursor < self.tokens.len() {
            let t = self.tokens[self.cursor].as_str();
            self.cursor += 1;
            Some(t)
        } else {
            None
        }
    }
    fn consumed(&self) -> usize { self.cursor }
}

fn drain(stream: &mut TokenStream) -> usize {
    let mut count = 0;
    while stream.next().is_some() { count += 1; }
    count
}

#[test]
fn test_drain_50k_tokens_within_budget() {
    let mut s = TokenStream::new(TOKEN_COUNT);
    let t0 = Instant::now();
    let n = drain(&mut s);
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < STREAM_BUDGET_MS, "took {}ms budget {}ms", elapsed, STREAM_BUDGET_MS);
    assert_eq!(n, TOKEN_COUNT);
}

#[test]
fn test_consumed_counter_matches_drained() {
    let mut s = TokenStream::new(1000);
    drain(&mut s);
    assert_eq!(s.consumed(), 1000);
}

#[test]
fn test_next_returns_none_after_exhaustion() {
    let mut s = TokenStream::new(2);
    let _ = s.next();
    let _ = s.next();
    assert!(s.next().is_none());
}

#[test]
fn test_token_prefix_is_tok() {
    let s = TokenStream::new(5);
    for t in &s.tokens { assert!(t.starts_with("tok")); }
}

#[test]
fn test_partial_drain_leaves_remainder() {
    let mut s = TokenStream::new(100);
    for _ in 0..40 { s.next(); }
    assert_eq!(s.consumed(), 40);
    let rest = drain(&mut s);
    assert_eq!(rest, 60);
}
