# Sampling Configuration

The `Sampler` in ancora-drift uses reservoir sampling to capture a fraction of
live production traces for offline evaluation.

## Configuration options

```rust
use ancora_drift::sampling::SamplingConfig;

let config = SamplingConfig {
    rate: 0.05,         // Sample 5% of requests (default)
    buffer_size: 1_000, // Maximum buffered traces before oldest is evicted
    seed: 42,           // RNG seed for reproducibility
};
```

### `rate`

A value between `0.0` (sample nothing) and `1.0` (sample everything). Higher
rates give more eval coverage but increase storage costs.

Recommended values:

| Traffic volume | Recommended rate |
|---|---|
| < 1 000 req/day | 0.20 |
| 1 000 - 10 000 | 0.10 |
| 10 000 - 100 000 | 0.05 |
| > 100 000 | 0.01 |

### `buffer_size`

Controls how many traces are held in memory before the oldest is evicted.
Size this to the number of traces you expect to accumulate between flush
cycles.

### `seed`

Setting a fixed seed makes sampling deterministic, which is useful in tests.
In production, vary the seed per process restart or use the default `42`.

## Flushing traces into evals

```rust
let mut sampler = Sampler::new(config);

// ... offer traces during request handling ...

// Periodically flush and feed into the eval pipeline:
let traces = sampler.drain();
for trace in traces {
    eval_pipeline.ingest(EvalCase {
        id: trace.id,
        input: trace.input,
        expected_output: None, // labelled asynchronously
        actual_output: trace.output,
        metadata: EvalMetadata { cost_micros: trace.cost_micros, .. },
    });
}
```

## Stratified sampling

If you need to ensure that rare tool usages are represented in the eval set,
apply a higher rate to those requests before offering them to the sampler:

```rust
let rate = if trace.tools_called.contains(&"rare_tool".to_string()) {
    1.0 // always capture
} else {
    config.rate
};
// Temporarily override by creating a per-request config:
let mut local_cfg = config.clone();
local_cfg.rate = rate;
let mut local_sampler = Sampler::new(local_cfg);
local_sampler.offer(trace);
```
