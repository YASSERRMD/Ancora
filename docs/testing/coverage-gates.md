# Test Coverage Gates

This document describes the coverage gates introduced in Phase 156. Coverage gates are Rust tests in `crates/ancora-core/tests/coverage_gate_*.rs` that enforce minimum coverage requirements across the whole test suite. They fail CI if a new suite forgets to register its tests, event types, or SDK languages.

## What each gate checks

| Gate file | What it enforces |
|---|---|
| `coverage_gate_module_list.rs` | All 57+ test module names present, snake_case, no duplicates |
| `coverage_gate_event_types.rs` | All 10 journal event types from `journal.proto` exercised |
| `coverage_gate_sdk_languages.rs` | All 6 SDKs (Rust, Go, Python, TS, .NET, Java) have single-agent, verifier, and HIL tests |
| `coverage_gate_vector_backends.rs` | All 11 vector backends have conformance tests |
| `coverage_gate_security_properties.rs` | All 11 security properties have dedicated sec_ tests |
| `coverage_gate_policy_properties.rs` | All 8 policy properties have dedicated policy_ tests |
| `coverage_gate_ci_workflows.rs` | All 5 phase CI workflow files registered |
| `coverage_gate_doc_pages.rs` | All 9 testing documentation pages listed |
| `coverage_gate_a2a_scenarios.rs` | All 6 A2A cross-language handoff pairs have tests |
| `coverage_gate_offline_only.rs` | All 8 suites documented as offline-only |
| `coverage_gate_replay_scenarios.rs` | All 19 replay scenarios mapped to det_ tests |
| `coverage_gate_chaos_scenarios.rs` | 9 chaos + 5 load + 5 reliability = 19 scenarios |
| `coverage_gate_min_test_count.rs` | Per-suite minimum test thresholds enforced |
| `coverage_gate_mcp_scenarios.rs` | MCP server/client cross-language pairs all tested |
| `coverage_gate_otel_spans.rs` | All 6 OTel span fields covered, canonical trace_id verified |
| `coverage_gate_cost_model.rs` | Cost formula ($3/M input, $15/M output) verified with 5 cases |
| `coverage_gate_structured_output.rs` | 8 structured output validation scenarios all tested |

## Running the coverage gate suite

```bash
cargo test -p ancora-core --test coverage_gate_module_list \
  --test coverage_gate_event_types \
  --test coverage_gate_sdk_languages \
  --test coverage_gate_vector_backends \
  --test coverage_gate_security_properties \
  --test coverage_gate_policy_properties \
  --test coverage_gate_ci_workflows \
  --test coverage_gate_doc_pages \
  --test coverage_gate_a2a_scenarios \
  --test coverage_gate_offline_only \
  --test coverage_gate_replay_scenarios \
  --test coverage_gate_chaos_scenarios \
  --test coverage_gate_min_test_count \
  --test coverage_gate_mcp_scenarios \
  --test coverage_gate_otel_spans \
  --test coverage_gate_cost_model \
  --test coverage_gate_structured_output
```

All gates run offline. No network calls, no live stores, no live keys.
