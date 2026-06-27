# Ancora .NET SDK Examples

Example tests demonstrating common patterns for the Ancora .NET SDK.
All tests skip gracefully when the native library is not present, so they can
run in CI without the Rust FFI library pre-built.

## Run

```bash
cd sdk/dotnet
dotnet test Ancora.Tests/Ancora.Tests.csproj --filter "FullyQualifiedName~Examples"
```

## Examples

| Example class | Description |
|---------------|-------------|
| `SingleAgentExampleTests` | Start a run, collect events, verify started/completed |
| `StructuredOutputExampleTests` | Derive JSON Schema from a C# record, validate agent output |
| `MultiAgentVerifierExampleTests` | Run primary and verifier agents concurrently |
| `HumanInLoopExampleTests` | Pause a run and resume with a decision |
| `StreamingChatExampleTests` | Consume events via `IAsyncEnumerable` |
| `RagPgvectorExampleTests` | Offline keyword retrieval injected as context (stands in for pgvector) |
| `McpToolExampleTests` | Define tool specs and wire them into `AgentSpec` |
| `GlmSelfHostExampleTests` | Configure GLM model variants and run them |
| `DurableRestartExampleTests` | Persist events to `RunJournal` and replay on restart |
| `CostOtelExampleTests` | Track spans, token estimates, and duration |

## Shared helpers

`SharedHelpers.cs` provides `RunJournal`, `Span`, and `TokenEstimator` utilities
used across the example test classes.
