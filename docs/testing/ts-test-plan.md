# TypeScript SDK Test Plan

## Overview

This document covers the offline test strategy for the Ancora TypeScript SDK.

## Running Tests

```bash
cd sdk/ts
npx jest                     # run all tests
npx jest --coverage          # run with coverage threshold from jest.config.js
npx jest phase145            # run Phase 145 e2e tests only
npx jest --testPathPattern='phase14[45]'  # run phases 144-145
```

## Test Layers

### Unit and Integration (Phase 144)

| File | Tests | Coverage |
|------|-------|----------|
| `phase144-addon-load.test.ts` | 10 | Addon load, Runtime create/free |
| `phase144-spec-roundtrip.test.ts` | 10 | AgentSpecSchema validation |
| `phase144-single-agent-run.test.ts` | 10 | For-await run lifecycle |
| `phase144-tool-handler.test.ts` | 10 | defineTool, ToolRegistry dispatch |
| `phase144-tool-error.test.ts` | 10 | Throwing tool handlers |
| `phase144-structured-output-zod.test.ts` | 10 | zodToInputSchema, Zod validation |
| `phase144-multi-agent-verifier.test.ts` | 10 | Two-agent pipeline |
| `phase144-human-in-loop.test.ts` | 10 | Suspend/resume via resumeRun |
| `phase144-streaming-iterator.test.ts` | 10 | Token stream iteration |
| `phase144-memory-readwrite.test.ts` | 10 | In-memory store CRUD |
| `phase144-rag-qdrant.test.ts` | 10 | Qdrant fixture retrieval |
| `phase144-provider-selection.test.ts` | 10 | Five provider model IDs |
| `phase144-cost-summary.test.ts` | 10 | Usage/cost events |
| `phase144-policy-residency.test.ts` | 10 | Residency block events |
| `phase144-mcp-tool-use.test.ts` | 10 | MCP tool call/result |
| `phase144-concurrent-runs.test.ts` | 9 | Concurrent isolation |
| `phase144-cancellation.test.ts` | 9 | Early break, AbortController |
| `phase144-error-normalization.test.ts` | 10 | Error code constants |
| `phase144-type-definitions.test.ts` | 11 | Public API surface exported |
| `phase144-wasm-path.test.ts` | 10 | WASM mock path |

**Total: ~198 unit tests**

### End-to-End and Reliability (Phase 145)

| File | Tests | Coverage |
|------|-------|----------|
| `phase145-e2e-single-agent.test.ts` | 10 | Full run lifecycle |
| `phase145-e2e-verifier.test.ts` | 10 | Two-node verifier pipeline |
| `phase145-e2e-human-in-loop.test.ts` | 10 | Approve/reject via resumeRun |
| `phase145-e2e-rag-qdrant.test.ts` | 10 | Qdrant tool e2e with ToolBridge |
| `phase145-e2e-mcp.test.ts` | 10 | MCP fixture e2e |
| `phase145-e2e-deepseek.test.ts` | 9 | DeepSeek/Qwen mock gateway |
| `phase145-conf-suite.test.ts` | 10 | Four canonical scenarios pass |
| `phase145-conf-journal.test.ts` | 10 | Journal ordering fixture |
| `phase145-rel-restart.test.ts` | 9 | Restart recovery via sidecar |
| `phase145-rel-zero-duplicate.test.ts` | 8 | Zero duplicate side effects |
| `phase145-rel-store-failure.test.ts` | 9 | Store failure recovery |
| `phase145-rel-rate-limit.test.ts` | 7 | Rate-limit burst handling |
| `phase145-rel-long-run.test.ts` | 7 | 50 runs, 100 runtimes, 500 ops |
| `phase145-sec-airgap.test.ts` | 8 | Air-gapped egress zero |
| `phase145-sec-mcp-auth.test.ts` | 10 | Unauthorized MCP refused |
| `phase145-perf-benchmark.test.ts` | 7 | Call latency bounds |
| `phase145-e2e-catalog-smoke.test.ts` | 8 | All 10 catalog examples |
| `phase145-e2e-wasm-sidecar.test.ts` | 8 | WASM client via sidecar |
| `phase145-e2e-cost-otel.test.ts` | 10 | Cost + OTel span emission |

**Total: ~170 e2e and reliability tests**

## Key Patterns

- All tests use `jest.mock('../ancora.node', ..., { virtual: true })` for offline execution
- Mock native module implements `startRun` / `pollRun` / `resumeRun` / `free` / `isFreed`
- Module-level counters per test file avoid cross-test pollution
- `beforeEach` clears run queues and resets counters
- `ToolBridge` is used for tests that exercise tool call/result flow
- `for await ... of handle` is the primary iteration primitive

## Coverage Gate

`jest.config.js` enforces:
- Lines: 70%
- Functions: 70%
- Statements: 70%
- Branches: 60%

Run with `npx jest --coverage` to verify.

## CI Configuration

See `.github/workflows/ts-e2e-ci.yml` for the workflow that runs all TypeScript tests.
