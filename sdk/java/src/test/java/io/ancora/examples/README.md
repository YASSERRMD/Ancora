# Ancora Java SDK Examples

Example tests demonstrating common patterns for the Ancora Java SDK.
All tests skip gracefully when the native library is not present using
`Assumptions.assumeTrue(AncoraNative.AVAILABLE, ...)`.

## Run

```bash
cd sdk/java
./gradlew test --tests "io.ancora.examples.*"
```

## Examples

| Test class | Description |
|------------|-------------|
| `SingleAgentExampleTest` | Start a run, collect events, verify started/completed |
| `StructuredOutputExampleTest` | Derive JSON Schema from a Java record, validate agent output |
| `MultiAgentVerifierExampleTest` | Run primary and verifier agents concurrently via `CompletableFuture` |
| `HumanInLoopExampleTest` | Pause a run and resume with a decision (string or bytes) |
| `StreamingChatExampleTest` | Consume events via the `Iterable<RunEvent>` loop |
| `RagMilvusExampleTest` | Offline keyword retrieval injected as context (stands in for Milvus) |
| `McpToolExampleTest` | Define tool specs and wire them into `AgentSpec` |
| `QwenGatewayExampleTest` | Configure Qwen model variants and verify distinct run IDs |
| `DurableRestartExampleTest` | Persist events to `RunJournal` and replay on restart |
| `CostOtelExampleTest` | Track spans, token estimates, and duration |

## Shared helpers

`SharedHelpers.java` provides `RunJournal`, `Span`, and `TokenEstimator` utilities
used across the example test classes.
