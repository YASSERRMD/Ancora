//! Integration tests: sampled traces feed the eval pipeline.

use crate::sampling::{SamplingConfig, Sampler, Trace};

fn make_trace(id: u32) -> Trace {
    Trace {
        id: id.to_string(),
        input: format!("question number {id}"),
        output: format!("answer number {id}"),
        cost_micros: 100 + id as u64,
        latency_ms: 50,
        provider: "openai".into(),
        tools_called: vec!["search".to_string()],
    }
}

#[test]
fn sampled_traces_become_eval_cases() {
    // Use rate=1.0 so all traces are captured.
    let cfg = SamplingConfig { rate: 1.0, buffer_size: 100, seed: 7 };
    let mut sampler = Sampler::new(cfg);

    for i in 0..20 {
        sampler.offer(make_trace(i));
    }

    let traces = sampler.drain();
    assert_eq!(traces.len(), 20, "all traces should be sampled at rate=1.0");

    // Verify traces have enough information to serve as eval cases.
    for trace in &traces {
        assert!(!trace.input.is_empty(), "input must not be empty");
        assert!(!trace.output.is_empty(), "output must not be empty");
        assert!(!trace.id.is_empty(), "id must not be empty");
    }
}

#[test]
fn sampler_respects_buffer_limit() {
    let cfg = SamplingConfig { rate: 1.0, buffer_size: 5, seed: 0 };
    let mut sampler = Sampler::new(cfg);
    for i in 0..10 {
        sampler.offer(make_trace(i));
    }
    // Buffer capped at 5 (oldest evicted).
    assert_eq!(sampler.buffered(), 5);
}

#[test]
fn partial_sampling_gives_subset() {
    let cfg = SamplingConfig { rate: 0.5, buffer_size: 1_000, seed: 99 };
    let mut sampler = Sampler::new(cfg);
    for i in 0..1_000 {
        sampler.offer(make_trace(i));
    }
    let n = sampler.buffered();
    // Expect roughly 500, allow wide tolerance.
    assert!(n > 300 && n < 700, "got {n} sampled traces");
}
