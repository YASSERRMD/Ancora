# Cross-Language Test Plan

This document describes the Ancora cross-language (xlang) conformance test suite. The suite proves that all six Ancora bindings -- Rust, Go, Python, TypeScript, .NET, and Java -- produce equivalent journals, interoperate over A2A, and agree on cost and OTel span formats.

## Guarantee

All six bindings must:
1. Produce structurally identical journals (same event sequence, same seq numbers, same run_id) for the same scenario.
2. Accept and produce A2A envelopes with a standard `protocol`, `sender`, `recipient`, `run_id`, and `payload` structure.
3. Compute identical cost summaries from the same token counts and rates.
4. Produce OTel spans that share the same trace_id across language boundaries.
5. Pass the shared fixture defined in `xlang_shared_fixture.rs` without modification.

Model-generated text in `result_json` and `output_json` is explicitly excluded from equality -- those fields are opaque.

## Test Files

### Rust (`crates/ancora-core/tests/`)

| File | What it covers |
|------|---------------|
| `xlang_single_agent_rust.rs` | Single agent scenario -- Rust baseline |
| `xlang_verifier_rust.rs` | Verifier scenario -- Rust baseline |
| `xlang_humaninloop_rust.rs` | Human-in-loop using `HumanDecisionRequested` + `HumanDecisionReceived` |
| `xlang_journal_equality.rs` | Structural journal equality across all six languages |
| `xlang_mcp_rust_python.rs` | Rust MCP server consumed by Python client |
| `xlang_a2a_identity.rs` | A2A identity headers for all six languages |
| `xlang_shared_fixture.rs` | Shared JSON fixture parsed and verified |
| `xlang_cost_parity.rs` | Cost formula is identical across languages |
| `xlang_otel_parity.rs` | OTel spans share trace_id and have correct parent chain |

### Go (`sdk/go/ancora/`)

| File | What it covers |
|------|---------------|
| `xlang_single_agent_test.go` | Single agent offline fixture |
| `xlang_verifier_test.go` | Verifier offline fixture |
| `xlang_humaninloop_test.go` | HIL offline fixture |
| `xlang_a2a_to_python_test.go` | Go to Python A2A handoff |
| `xlang_mcp_go_ts_test.go` | Go MCP server consumed by TS client |

### Python (`sdk/python/tests/`)

| File | What it covers |
|------|---------------|
| `test_xlang_single_agent.py` | Single agent offline fixture |
| `test_xlang_verifier.py` | Verifier offline fixture |
| `test_xlang_humaninloop.py` | HIL offline fixture |
| `test_xlang_a2a_to_go.py` | Python to Go A2A handoff |

### TypeScript (`sdk/ts/__tests__/`)

| File | What it covers |
|------|---------------|
| `xlang-single-agent.test.ts` | Single agent offline fixture |
| `xlang-verifier.test.ts` | Verifier offline fixture |
| `xlang-humaninloop.test.ts` | HIL offline fixture |
| `xlang-a2a-to-dotnet.test.ts` | TS to .NET A2A handoff |

### .NET (`sdk/dotnet/Ancora.Tests/`)

| File | What it covers |
|------|---------------|
| `XlangSingleAgentTests.cs` | Single agent offline fixture |
| `XlangVerifierTests.cs` | Verifier offline fixture |
| `XlangHumanInLoopTests.cs` | HIL offline fixture |

### Java (`sdk/java/src/test/java/io/ancora/`)

| File | What it covers |
|------|---------------|
| `Phase152XlangSingleAgentTest.java` | Single agent offline fixture |
| `Phase152XlangVerifierTest.java` | Verifier offline fixture |
| `Phase152XlangHumanInLoopTest.java` | HIL offline fixture |
| `Phase152A2aJavaRustTest.java` | Java to Rust A2A handoff |

## Event Contract

Every binding must produce this event sequence for the single-agent scenario:

```
seq=0  kind=started     run_id=<id>  spec_type=AgentSpec
seq=1  kind=activity    run_id=<id>  activity_key=main-agent  activity_kind=agent-output
seq=2  kind=completed   run_id=<id>
```

`result_json` inside the activity event is excluded from equality -- it contains model output.

## A2A Envelope Contract

```json
{
  "protocol": "a2a/1.0",
  "sender":    {"lang": "<source-lang>", "sdk_version": "0.3.0"},
  "recipient": {"lang": "<target-lang>", "sdk_version": "0.3.0"},
  "run_id":    "<shared-run-id>",
  "payload":   {}
}
```

`run_id` must be the same in both the request and the response envelope.

## Running the suite

```bash
# Rust
cargo test -p ancora-core --test xlang_single_agent_rust --test xlang_journal_equality ...

# Go
cd sdk/go && go test ./ancora/... -run TestXlang

# Python
cd sdk/python && pytest tests/test_xlang_*.py

# TypeScript
cd sdk/ts && npx jest --testPathPattern="xlang"

# .NET
cd sdk/dotnet && dotnet test --filter "Xlang"

# Java
cd sdk/java && ./gradlew test --tests "io.ancora.Phase152*"
```

## Offline guarantee

All xlang tests run offline. No live HTTP calls, no live MCP servers, no live A2A peers. All fixtures are embedded in the test files. The CI workflow `xlang-conformance-ci.yml` runs all six languages in the same job without network access.
