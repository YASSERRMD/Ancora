# Example Completeness and Parity Report

This report covers the example parity tests introduced in Phase 158. Each test verifies that the same example scenario produces equivalent results across all supported languages and configurations.

## Parity test files

| File | Scenario | Languages / Configs |
|---|---|---|
| `example_parity_single_agent.rs` | Single-agent run | All 6 SDKs |
| `example_parity_verifier.rs` | Verifier pattern | All 6 SDKs |
| `example_parity_hil.rs` | Human-in-loop | All 6 SDKs |
| `example_parity_vector_rag.rs` | Vector RAG | 4 backends |
| `example_parity_mcp.rs` | MCP tool use | 4 server/client pairs |
| `example_parity_streaming.rs` | Streaming tokens | All 6 SDKs |
| `example_parity_cost.rs` | Cost tracking | All 6 SDKs |
| `example_parity_otel.rs` | OTel spans | Rust + Go |
| `example_parity_local_provider.rs` | Local-first provider | Ollama + LMStudio |
| `example_parity_structured_output.rs` | Structured output | All 6 SDKs |
| `example_parity_policy.rs` | Policy enforcement | All 6 SDKs |
| `example_parity_durability.rs` | Crash and resume | All 6 SDKs |
| `example_parity_a2a.rs` | A2A handoff | 4 language pairs |
| `example_parity_error_handling.rs` | Error handling | All 6 SDKs |
| `example_parity_chinese_providers.rs` | Qwen, GLM, DeepSeek | 3 providers, 5+1+3 SDKs |
| `example_parity_multi_agent.rs` | Multi-agent parallel | 4 SDKs |
| `example_parity_edge_deployment.rs` | Edge/Wasm deployment | Rust, TS, Go |

## What parity means

For each scenario, the tests verify that:
- The same input produces the same output shape (same field names, same structure)
- The same event sequence is observed (`started -> ... -> completed`)
- The same cost formula applies ($3/M input, $15/M output)
- The same trace_id/span_id structure appears in OTel output
- The same policy decisions are made (same model and region rules)
- The same error types are surfaced when things go wrong

All parity tests run offline.
