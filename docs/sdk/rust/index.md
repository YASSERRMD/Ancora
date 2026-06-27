# Rust SDK

The Ancora Rust SDK is the native SDK. It directly uses `ancora-core` and
`ancora-proto` and has zero FFI overhead.

## Pages

| Page | Description |
|------|-------------|
| [Install](install.md) | Cargo dependency setup and feature flags |
| [Quickstart](quickstart.md) | Minimal single-agent example |
| [Tools](tools.md) | Registering Rust closures as agent tools |
| [Structured output](structured-output.md) | `serde` deserialization from run output |
| [Multi-agent](multi-agent.md) | Graph-based orchestration with `GraphSpec` |
| [Verifier](verifier.md) | Primary and verifier pattern with `JoinSet` |
| [Human-in-the-loop](human-in-the-loop.md) | Suspend and resume a run |
| [Streaming](streaming.md) | `Run` event iterator and cancellation |
| [Memory and RAG](memory-and-rag.md) | Context injection and LanceDB retrieval |
| [Providers](providers.md) | Ollama, Anthropic, OpenAI, Gemini |
| [Chinese providers](chinese-providers.md) | GLM, Qwen, DeepSeek, regional endpoints |
| [Vector stores](vector-stores.md) | LanceDB, pgvector, Milvus, Qdrant |
| [Durability](durability.md) | `JournalStore`, `MemoryStore`, replay |
| [Observability](observability.md) | Cost tracking and OTEL export |
| [Policy](policy.md) | Data residency rules and effect classes |
| [MCP and A2A](mcp-and-a2a.md) | Interoperability |
| [Edge deployment](edge-deployment.md) | `cargo build --release`, musl, WASI |
| [Testing](testing.md) | Offline `tokio::test` patterns |
| [Troubleshooting](troubleshooting.md) | Common errors and fixes |
| [API reference](api-reference.md) | Full type and trait listing |
| [Migration](migration.md) | From `async-openai`, LLM crate |
| [Error handling](error-handling.md) | Error types, retry, parallel collection |
| [Concurrency](concurrency.md) | `JoinSet`, `Semaphore`, `FuturesUnordered` |
| [Configuration](configuration.md) | Environment variables and `RuntimeOptions` |

## Requirements

- Rust 1.75 or later (2021 edition)
- Add to `Cargo.toml`:
  ```toml
  [dependencies]
  ancora-core = { git = "https://github.com/ancora-ai/ancora" }
  ancora-proto = { git = "https://github.com/ancora-ai/ancora" }
  tokio = { version = "1", features = ["full"] }
  ```

## Key types

| Type | Crate | Description |
|------|-------|-------------|
| `Runtime` | `ancora-core` | Entry point; spawns runs |
| `Run` | `ancora-core` | In-flight agent run |
| `RunEvent` | `ancora-core` | Streamed event (Token, Completed, etc.) |
| `AgentSpec` | `ancora-core` | Model, instructions, tools, policy |
| `ToolSpec` | `ancora-core` | Tool name, description, schema, handler |
| `EffectClass` | `ancora-core` | `Read`, `Write`, `None` |
| `MemoryStore` | `ancora-core` | In-memory journal (for tests) |
| `JournalStore` | `ancora-core` | Persistent journal trait |
| `PolicySpec` | `ancora-core` | Data residency and write-tool limits |
| `GraphSpec` | `ancora-core` | Multi-agent graph definition |

See [Install](install.md) for full setup.
