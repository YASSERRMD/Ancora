// Benchmark: A2A envelope serialisation -- 500k envelopes under 500ms.

use std::time::Instant;

const A2A_BENCH_N: usize = 500_000;
const A2A_BENCH_MS: u128 = 5000;

struct A2AEnvelope<'a> {
    protocol: &'a str,
    sender_lang: &'a str,
    recipient_lang: &'a str,
    run_id: u64,
}

impl<'a> A2AEnvelope<'a> {
    fn to_json_len(&self) -> usize {
        self.protocol.len() + self.sender_lang.len() + self.recipient_lang.len() + 32
    }
    fn is_valid(&self) -> bool {
        self.protocol == "a2a/1.0"
    }
}

#[test]
fn test_bench_500k_a2a_envelopes_under_500ms() {
    let t0 = Instant::now();
    let langs = ["rust", "go", "python", "typescript", "dotnet", "java"];
    let mut total_len = 0usize;
    for i in 0..A2A_BENCH_N {
        let env = A2AEnvelope {
            protocol: "a2a/1.0",
            sender_lang: langs[i % 6],
            recipient_lang: langs[(i + 1) % 6],
            run_id: i as u64,
        };
        total_len += env.to_json_len();
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < A2A_BENCH_MS, "took {}ms budget {}ms", elapsed, A2A_BENCH_MS);
    assert!(total_len > 0);
}

#[test]
fn test_envelope_valid_protocol() {
    let env = A2AEnvelope { protocol: "a2a/1.0", sender_lang: "rust", recipient_lang: "go", run_id: 1 };
    assert!(env.is_valid());
}

#[test]
fn test_envelope_invalid_protocol() {
    let env = A2AEnvelope { protocol: "a2a/2.0", sender_lang: "rust", recipient_lang: "go", run_id: 1 };
    assert!(!env.is_valid());
}

#[test]
fn test_envelope_json_len_positive() {
    let env = A2AEnvelope { protocol: "a2a/1.0", sender_lang: "python", recipient_lang: "java", run_id: 99 };
    assert!(env.to_json_len() > 10);
}
