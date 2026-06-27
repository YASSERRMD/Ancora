# Java SDK

The Ancora Java SDK targets Java 17+ and uses JNI to call the native
Ancora engine via `AncoraNative`. Events are delivered as
`Iterable<RunEvent>` (sealed interface with pattern-matching records) and
structured output uses Jackson `@JsonProperty`.

## Pages

| Page | Description |
|------|-------------|
| [Install](install.md) | Maven / Gradle dependency and native library setup |
| [Quickstart](quickstart.md) | Minimal single-agent example |
| [Tools](tools.md) | Registering lambdas as agent tools |
| [Structured output](structured-output.md) | Jackson `@JsonProperty` record output |
| [Multi-agent](multi-agent.md) | Graph-based orchestration |
| [Verifier](verifier.md) | Primary and verifier consensus |
| [Human-in-the-loop](human-in-the-loop.md) | Suspend and resume a run |
| [Streaming](streaming.md) | `Iterable<RunEvent>` iteration |
| [Memory and RAG](memory-and-rag.md) | Context injection and Milvus retrieval |
| [Providers](providers.md) | Ollama, Anthropic, OpenAI, Azure |
| [Qwen regional endpoints](qwen-regional.md) | GLM, Qwen, DeepSeek |
| [Vector stores](vector-stores.md) | Milvus, pgvector, Qdrant, Weaviate |
| [Durability](durability.md) | Persistent SQLite journal |
| [Observability](observability.md) | Cost tracking and OTEL export |
| [Policy](policy.md) | Data residency rules |
| [MCP and A2A](mcp-and-a2a.md) | Interoperability |
| [Deployment](deployment.md) | Fat-JAR, Docker, Spring Boot |
| [Testing](testing.md) | Offline JUnit 5 patterns |
| [Troubleshooting](troubleshooting.md) | Common errors and fixes |
| [API reference](api-reference.md) | Full symbol index |
| [Migration](migration.md) | From LangChain4j and Semantic Kernel Java |

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
