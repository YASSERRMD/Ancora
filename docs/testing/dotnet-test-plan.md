# .NET SDK Test Plan

## Overview

The .NET SDK test suite covers the full binding surface from native load through end-to-end scenario execution. Tests run offline by catching `DllNotFoundException` when the native `ancora_ffi` library is absent, allowing the CI to build the library before running the full integration suite.

## Test structure

Tests live in `sdk/dotnet/Ancora.Tests/` and use xUnit 2.x.

### Unit tests (Phase 146)

| File | Description |
|---|---|
| `Phase146NativeLoadDisposeTests.cs` | IDisposable contract, double-dispose safety |
| `Phase146SpecRoundTripTests.cs` | AgentSpec and GraphSpec JSON serialization |
| `Phase146SingleAgentRunTests.cs` | RunHandle lifecycle, event ordering |
| `Phase146ToolAttributeExecutionTests.cs` | [Tool] attribute discovery and dispatch |
| `Phase146ToolErrorPropagationTests.cs` | Throwing handlers, AncorException codes |
| `Phase146StructuredOutputTests.cs` | Record value equality and with-expressions |
| `Phase146MultiAgentVerifierTests.cs` | Two-agent pipeline, parallel collection |
| `Phase146HumanInLoopTests.cs` | Resume(), ResumeAndCollectAsync() |
| `Phase146StreamingTests.cs` | IAsyncEnumerable, cancellation |
| `Phase146MemoryReadWriteTests.cs` | InMemoryStore, 500-op stress |
| `Phase146RagRetrievalTests.cs` | pgvector fixture, BuildSchema reflection |
| `Phase146ProviderSelectionTests.cs` | Provider model ID constants |
| `Phase146CostSummaryTests.cs` | CostEvent record, accumulation |
| `Phase146PolicyResidencyTests.cs` | Blocked/allowed regions |
| `Phase146McpToolUseTests.cs` | Auth token, unauthorized error |
| `Phase146ConcurrentRunsTests.cs` | 50-run and 100-RT-cycle stress |
| `Phase146CancellationTokenTests.cs` | CancellationToken propagation |
| `Phase146ErrorNormalizationTests.cs` | AncorErrorCode values |
| `Phase146SafeHandleTests.cs` | RuntimeHandle extends SafeHandle |
| `Phase146NullableAnnotationTests.cs` | Nullable/non-nullable property types |

### End-to-end and reliability tests (Phase 147)

| File | Description |
|---|---|
| `Phase147E2eSingleAgentTests.cs` | Full lifecycle: started -> completed |
| `Phase147E2eVerifierTests.cs` | Two-agent verifier pipeline |
| `Phase147E2eHumanInLoopTests.cs` | Approve/reject resume cycle |
| `Phase147E2eRagPgvectorTests.cs` | pgvector fixture tool integration |
| `Phase147E2eMcpTests.cs` | MCP tool with auth token |
| `Phase147E2eGlmSelfHostTests.cs` | GLM self-hosted via mock gateway |
| `Phase147ConfSuiteTests.cs` | Four conformance scenarios |
| `Phase147ConfJournalTests.cs` | Journal fixture matching |
| `Phase147RelRestartTests.cs` | Create-free-create recovery cycles |
| `Phase147RelZeroDuplicateTests.cs` | No duplicate run IDs or side effects |
| `Phase147RelStoreFailureTests.cs` | FailableStore recovery |
| `Phase147RelRateLimitTests.cs` | Rate-limit fixture, backoff |
| `Phase147RelLongRunTests.cs` | 50-run, 100-RT-cycle stability |
| `Phase147SecAirgapTests.cs` | No external URLs or API keys in events |
| `Phase147SecMcpAuthTests.cs` | Unauthorized access refused |
| `Phase147PerfBenchmarkTests.cs` | Wall time bounds for operations |
| `Phase147E2eCatalogSmokeTests.cs` | 10 catalog examples |
| `Phase147E2eVectorStoreParityTests.cs` | pgvector vs LanceDB parity |
| `Phase147E2eCostOtelTests.cs` | OTel span and cost accumulation |

## How to run

```bash
cd sdk/dotnet

# Unit tests only (no native library needed for type/reflection tests):
dotnet test --no-build --configuration Debug

# Full suite with coverage (requires native library):
cargo build -p ancora-ffi --release
cp ../../target/release/libancora_ffi.so Ancora.Tests/
LD_LIBRARY_PATH=Ancora.Tests dotnet test --collect:"XPlat Code Coverage"
```

## Coverage gate

CI enforces a 60% line coverage gate via Cobertura XML parsing. The threshold is configured in `.github/workflows/dotnet-ci.yml`.

## Offline guarantee

All fixtures are in-process (no network calls). Integration tests that require the native library catch `DllNotFoundException` and skip gracefully when run in environments without the Rust FFI build.
