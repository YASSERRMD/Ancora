# Changelog

## 0.6.0 (2026-06-29)

- [Added] ancora-oe-milestone crate: obs and eval release checkpoint
- [Added] Histogram bucket support in Rust and Python SDKs (GA)
- [Added] ancora-evallib: offline eval computation library
- [Added] ancora-conteval: continuous evaluation pipeline
- [Added] ancora-obssdk: unified observability SDK wrapper
- [Added] ancora-obsint: observability integration test harness
- [Added] ancora-oepar: obs and eval parity validation suite
- [Added] ancora-oe-docs: consolidated obs and eval documentation
- [Added] ancora-oe-e2e: end-to-end obs and eval scenario tests
- [Changed] Renamed `otel_endpoint` to `exporter_endpoint` in config
- [Changed] Default metrics retention extended to 30 days
- [Changed] Eval gate threshold syntax now supports `p` percentile prefix
- [Fixed] Race condition in metric flush under high concurrency
- [Fixed] Trace context not propagated in async Python tasks
- [Performance] Reduced allocations in hot tracer path by 40%
- [Performance] Batch eval submission throughput improved 3x
- [Docs] Per-language quickstart guides for observability setup
- [Docs] Privacy posture summary for GDPR/SOC2 compliance
- [Docs] Self-hosted observability deployment guide
- [Docs] Metrics and evals catalog index

## 0.5.0 (2026-04-15)

- [Added] ancora-evalgate: CI eval gating
- [Added] ancora-evalrun: eval execution engine
- [Added] ancora-evals: eval framework core
- [Added] ancora-trace: distributed tracing primitives
- [Added] ancora-observability: observability stack entrypoint
