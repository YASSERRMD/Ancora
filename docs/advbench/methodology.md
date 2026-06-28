# Advanced Benchmark Methodology

## Overview

`ancora-advbench` measures the in-process overhead and cost of all 10 advanced
capability areas.  The harness is deterministic, offline, and reproducible
across runs on the same machine.

## What is measured

| Benchmark | Operation | Metric |
|---|---|---|
| planner | 1,000 x PlanningMetric::score(100 steps, 75 matched) | elapsed_ns, quality |
| reflection | 10,000 x ReflectionMetric::score(200-char, 300-char) | elapsed_ns, quality |
| routing | 10,000 x RoutingMetric::score(quality=0.9, varying cost) | elapsed_ns, quality |
| optimization | 20 x PlanningMetric::score(500 steps, 400 matched) | elapsed_ns, quality |
| memory_consolidation | 100 x ConsolidationJob.run(50 episodic entries) | elapsed_ns, promoted count |
| coordination | 1,000 x CoordJournal.record | elapsed_ns, journal size |
| guardrail | 1,000 x InjectionInputGuardrail.check_input | elapsed_ns, blocked count |
| reasoning | 500 x CitationStore.add + ReasoningJournal.record | elapsed_ns, citation count |
| lh_checkpoint | CheckpointCadence(10) over 500 ticks | elapsed_ns, checkpoint count |
| skills_jit | 200 x SkillRegistry.load | elapsed_ns, skill count |

## Timing

`std::time::Instant` (monotonic) is used for all measurements.  No wall-clock
time is used so results are deterministic between runs on the same machine.

## Regression gating

The `BASELINE` constants in `thresholds.rs` are conservative (measured typical
values multiplied by 2).  CI gates on `elapsed_ns < baseline * 2`, giving 4x
total headroom over the typical measurement.

## Reproducibility

All benchmarks use pure arithmetic or in-memory data structures.  No random
number generators, network calls, or external state are used.  Running the
same bench suite twice on the same machine produces equal `token_units` and
`quality` values.
