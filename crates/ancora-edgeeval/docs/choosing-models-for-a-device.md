# Choosing Models for a Device

ancora-edgeeval's `recommend` module matches SLM candidates to device profiles.

## Device Profiles

Four built-in profiles cover the main edge categories:

| Profile | RAM | Compute | Battery | Max Latency/Token |
|---------|-----|---------|---------|-------------------|
| `microcontroller` | 256 MiB | 0.5 GOPS | 500 mWh | 500 ms |
| `mobile` | 4096 MiB | 20 GOPS | 4000 mWh | 100 ms |
| `laptop` | 16384 MiB | 100 GOPS | 50000 mWh | 50 ms |
| `edge-server` | 65536 MiB | 1000 GOPS | mains | 10 ms |

Create a custom profile with `DeviceProfile::new(...)`.

## Model Candidates

Add candidates via `DeviceRecommender::add_candidate`. Use the built-in helpers
to estimate memory and latency:

```rust
use ancora_edgeeval::{SmallModel, ModelCandidate, DeviceRecommender, DeviceProfile};

let model = SmallModel::new("phi-2-int8", 2_700, 8);
let mem = ModelCandidate::estimate_memory_mib(&model);
let lat = ModelCandidate::estimate_latency_ms(&model, 20.0); // 20 GOPS
let candidate = ModelCandidate::new(model, mem, lat);

let mut rec = DeviceRecommender::new();
rec.add_candidate(candidate);

let device = DeviceProfile::mobile();
let result = rec.recommend(&device);
println!("{:?}", result.recommended_model_name);
```

## Recommendation Logic

The recommender filters candidates that fit within:
1. Device RAM (`estimated_memory_mib <= ram_mib`)
2. Latency budget (`estimated_latency_ms <= max_latency_per_token_ms`)

Among eligible candidates it selects the largest by parameter count, using size
as a quality proxy. Add your own scoring logic by inspecting `Recommendation`.

## Quantization Guidance

For memory-constrained devices prefer INT4 or INT8 quantization. Use
`QuantTradeoffEval` to confirm quality degradation is within acceptable bounds
before deploying.

| Format | Compression vs FP32 | Typical Quality Drop |
|--------|---------------------|----------------------|
| FP16 | 2x | ~0% |
| INT8 | 4x | 1-3% |
| INT4/NF4 | 8x | 3-8% |

## Integration with CI

Run `cargo test -p ancora-edgeeval` in your CI pipeline. All tests are offline and
deterministic, making them safe for air-gapped build environments.
