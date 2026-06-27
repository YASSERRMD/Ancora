# Python SDK Test Plan

## Overview

This document covers the offline test strategy for the Ancora Python SDK.

## Running Tests

```bash
cd sdk/python
make test          # run all tests
make coverage      # run with coverage threshold (default COVER_MIN=70)
COVER_MIN=80 make coverage  # raise the threshold
```

## Test Layers

### Unit and Integration (Phase 142)

| File | Tests | Coverage |
|------|-------|----------|
| `test_phase142_import_runtime.py` | 10 | Runtime creation/destruction |
| `test_phase142_spec_roundtrip.py` | 10 | AgentSpec wire round-trip |
| `test_phase142_single_agent_async.py` | 10 | Async run lifecycle |
| `test_phase142_tool_decorator.py` | 10 | `@tool` decorator and dispatch |
| `test_phase142_tool_error_propagation.py` | 10 | Tool error paths |
| `test_phase142_structured_output_pydantic.py` | 10 | Pydantic output schema |
| `test_phase142_multi_agent_verifier.py` | 11 | ConformanceSuite two-node |
| `test_phase142_human_in_loop.py` | 9 | Suspend/resume |
| `test_phase142_streaming_async.py` | 10 | Async generator streaming |
| `test_phase142_memory_readwrite.py` | 13 | MemoryStore CRUD |
| `test_phase142_rag_pgvector.py` | 10 | PgVector fixture retrieval |
| `test_phase142_provider_selection.py` | 10 | Five provider model IDs |
| `test_phase142_cost_summary.py` | 10 | Usage/cost events |
| `test_phase142_policy_residency.py` | 10 | Residency block events |
| `test_phase142_mcp_tool_use.py` | 10 | MCP tool call/result |
| `test_phase142_concurrent_runs.py` | 8 | Concurrent isolation |
| `test_phase142_cancellation.py` | 8 | asyncio.CancelledError paths |
| `test_phase142_error_normalization.py` | 10 | AncorError constants |
| `test_phase142_typing_stubs.py` | 12 | Public API surface |
| `test_phase142_gil_release.py` | 7 | Threading / GIL release |

**Total: 198 unit tests**

### End-to-End and Reliability (Phase 143)

| File | Tests | Coverage |
|------|-------|----------|
| `test_phase143_e2e_single_agent.py` | 10 | Full single-agent lifecycle |
| `test_phase143_e2e_verifier.py` | 10 | Two-node verifier pipeline |
| `test_phase143_e2e_human_in_loop.py` | 9 | Approve/reject lifecycle |
| `test_phase143_e2e_rag_pgvector.py` | 10 | PgVector e2e with agent |
| `test_phase143_e2e_mcp.py` | 10 | MCP tool e2e |
| `test_phase143_e2e_qwen_regional.py` | 10 | Qwen regional mock |
| `test_phase143_conf_suite.py` | 10 | ConformanceSuite all scenarios |
| `test_phase143_conf_journal.py` | 10 | Journal ordering fixture |
| `test_phase143_rel_restart.py` | 8 | Restart recovery |
| `test_phase143_rel_zero_duplicate.py` | 8 | Zero duplicate side effects |
| `test_phase143_rel_store_failure.py` | 9 | Store failure recovery |
| `test_phase143_rel_rate_limit.py` | 7 | Rate-limit burst handling |
| `test_phase143_rel_long_run.py` | 7 | 50 sequential runs, 100 runtimes |
| `test_phase143_sec_airgap.py` | 9 | Air-gapped egress |
| `test_phase143_sec_mcp_auth.py` | 10 | Unauthenticated MCP refused |
| `test_phase143_perf_call_overhead.py` | 7 | Call latency bounds |
| `test_phase143_e2e_catalog_smoke.py` | 8 | All 10 catalog examples |
| `test_phase143_e2e_vector_store_parity.py` | 10 | LanceDB vs PgVector parity |
| `test_phase143_e2e_cost_otel.py` | 10 | Cost + OTel span emission |

**Total: 172 e2e and reliability tests**

## Key API Facts

- `ancora.Runtime()` -- no args; `rt.free()` / `rt.is_freed` / context manager
- `AgentSpec` is a Pydantic model; fields: `name`, `model_id`, `tools`, `system_prompt`, `output_schema_json`, `require_approval`
- `ancora.Agent(rt, spec).run()` is async; returns `Run`
- `Run.drain_events()` is async; returns `list[bytes]`
- `Run.stream_events()` is an async generator
- `ancora.ConformanceSuite()` -- no args; `await suite.run_all(rt)` returns `Dict[str, bool]`
- `MemoryStore` -- in-memory key-value; `.write(k,v)`, `.read(k, default)`, `.delete(k)`, `.clear()`, `.update(d)`, `.pop(k, default)`, `.keys`, `.values`
- `@tool` decorator -- wraps a function; `.call_with_json(json_str)` dispatches it
- `ToolRegistry` -- `.register(tool)`, `.get(name)`, `.dispatch(name, json_str)`

## Offline Strategy

All tests run without live network access:
- Fixture dicts substitute for real API responses
- `socket.setdefaulttimeout(0.001)` probes for network in airgap tests
- `asyncio.gather` and `asyncio.CancelledError` paths use the offline mock runtime
- No API keys required; CI runs with `ANCORA_OFFLINE=1` (default)

## CI Configuration

See `.github/workflows/python-e2e-ci.yml` for the workflow that runs these tests.
