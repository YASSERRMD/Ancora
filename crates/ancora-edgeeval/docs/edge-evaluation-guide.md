# Edge Evaluation Guide

ancora-edgeeval provides a suite of offline, reproducible evaluations tuned for
edge constraints and small language models (SLMs).

## Overview

Edge deployments impose hard constraints not present in cloud inference:
- Limited RAM (256 MiB to 16 GiB)
- Low sustained compute (0.5 to 100+ GOPS)
- Battery budgets (mobile and IoT devices)
- No reliable network connectivity

ancora-edgeeval measures all four dimensions plus model quality, so you can make
principled tradeoffs rather than guessing.

## Evaluation Dimensions

| Dimension | Module | Key Metric |
|-----------|--------|------------|
| Capability | `model` | Pass rate on small-model suite |
| Latency | `runtime` | P50/P95 tokens-per-second |
| Memory | `runtime` | Total MiB (weights + KV + activations) |
| Power | `runtime` | Tokens per joule |
| Quantization tradeoff | `quant` | Tradeoff score |
| SLM reliability | `reliability` | Consistency + calibration |

## Quick Start

```rust
use ancora_edgeeval::{
    SmallModelSuite, CapabilitySample, TaskCategory,
    LatencyEvaluator, MemoryFootprint, PowerProxy,
    OfflineDataset, OfflineConfig, OfflineEvalRunner,
    EdgeEvalReport,
};

// 1. Build capability suite.
let mut suite = SmallModelSuite::new();
suite.add(CapabilitySample::new("q1", TaskCategory::Qa, "What is 2+2?", "4"));

// 2. Evaluate offline.
let outputs = [("q1", "4")];
let results = suite.evaluate_exact(&outputs);
let pass_rate = SmallModelSuite::pass_rate(&results);

// 3. Measure memory.
let footprint = MemoryFootprint::new("my-model", 600_000_000, 50_000_000, 10_000_000);

// 4. Generate report.
let mut report = EdgeEvalReport::new("My Edge Eval");
```

## Running Evaluations

All evaluations run fully offline. No network calls are made. Use `OfflineConfig`
to cap sample count and set a deterministic seed:

```rust
let config = OfflineConfig::new()
    .with_strict_offline(true)
    .with_max_samples(50)
    .with_seed(42);
let runner = OfflineEvalRunner::new(config);
let dataset = OfflineDataset::builtin_smoke();
let scores = runner.run(&dataset, &[]);
```

## Quantization Tradeoffs

Use `QuantTradeoffEval` to compare INT8/INT4 quantized variants against an FP32
baseline. The tradeoff score balances memory savings against quality degradation.

## Reporting

`EdgeEvalReport::render_text()` produces a markdown table summarising all models
and highlights the recommended model by edge score.
