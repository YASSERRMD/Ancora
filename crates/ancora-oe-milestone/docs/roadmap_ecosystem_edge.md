# Roadmap: Ecosystem and Edge (Phase 250)

This document outlines the planned work following the observability and eval
milestone.

## Themes

### 1. WebAssembly Eval Runner

- Compile the eval execution engine to WASM
- Run evals in browser and edge environments without a sidecar
- Target: Beta in Phase 250

### 2. Edge-Native Metric Aggregation

- Pre-aggregate metrics at the edge node before central export
- Reduce egress bandwidth by up to 80% in high-frequency scenarios
- DeltaTemporality support for all metric instruments
- Target: GA in Phase 250

### 3. Multi-Model Eval Harness

- Compare outputs from multiple LLM providers in a single eval run
- Provider-agnostic scoring API
- Built-in adapters: Anthropic, OpenAI, Mistral, local Ollama
- Target: Beta in Phase 250

### 4. Ecosystem Integrations

- Grafana plugin for Ancora eval results
- Datadog metric forwarding adapter
- OpenTelemetry Collector contrib exporter
- Target: Preview in Phase 250

### 5. SDK Completions

- Go SDK: promote log correlation from Beta to GA
- Go SDK: add privacy label scrubbing (currently Planned)
- All SDKs: mTLS support for self-hosted exporter

## Timeline

| Phase | Target Date | Theme |
| --- | --- | --- |
| 240 (current) | 2026-06-29 | Obs and eval milestone |
| 250 | 2026-09-30 | Ecosystem and edge |
| 260 | 2026-12-31 | Enterprise scale and compliance |

Last updated: 2026-06-29
