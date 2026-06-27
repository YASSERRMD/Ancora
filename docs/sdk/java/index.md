# Java SDK

The Ancora Java SDK targets Java 17+ and uses JNI to call the native
Ancora engine via `AncoraNative`.

## Pages

| Page | Description |
|------|-------------|
| Install | Maven / Gradle dependency and native library setup |
| Quickstart | Minimal single-agent example |
| Tools | Registering lambdas as agent tools |
| Structured output | Jackson `@JsonProperty` record output |
| Multi-agent | Graph-based orchestration |
| Verifier | Primary and verifier pattern |
| Human-in-the-loop | Suspend and resume a run |
| Streaming | `Iterable<RunEvent>` iteration |
| Memory and RAG | Context injection and Milvus retrieval |
| Providers | Ollama, Anthropic, OpenAI, Azure |
| Chinese providers | GLM, Qwen, DeepSeek |
| Vector stores | LanceDB, pgvector, Milvus, Qdrant |
| Durability | Persistent SQLite journal |
| Observability | Cost tracking and OTEL export |
| Policy | Data residency rules |
| MCP and A2A | Interoperability |
| Edge deployment | Fat-JAR with bundled native library |
| Testing | Offline JUnit 5 patterns |
| Troubleshooting | Common errors and fixes |
| API reference | Full Javadoc symbol index |

## Requirements

- Java 17 or later
- A Rust toolchain to build the native library (or a pre-built binary)
- Add `io.ancora:ancora-sdk` to your build file

## Key types

| Type | Description |
|------|-------------|
| `AncoraNative` | Low-level JNI bridge; `AncoraNative.AVAILABLE` boolean |
| `Runtime (AutoCloseable)` | Engine handle |
| `Agent (AutoCloseable)` | Runs agent tasks |
| `AgentSpec` | `(model, instructions, tools, maxTokens, temperature)` |
| `RunHandle` | Handle to an in-flight run |
| `RunEvent` | Sealed interface: `Started`, `Token`, `Completed`, `Resumed`, `ToolCall` |
| `ToolSpec` | Tool name, description, input schema |
| `ToolInputSchema` | JSON Schema wrapper |
| `ToolInputProperty` | Individual property descriptor |

Tests use `Assumptions.assumeTrue(AncoraNative.AVAILABLE, "...")` to skip
gracefully when the native library is absent.

See [Install](install.md) for full setup.
