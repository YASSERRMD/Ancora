# Rust SDK

The Ancora Rust SDK is the native SDK. It directly uses `ancora-core` and
`ancora-proto` and has zero FFI overhead.

## Pages

| Page | Description |
|------|-------------|
| Install | Cargo dependency setup |
| Quickstart | Minimal single-agent example |
| Tools | Registering Rust closures as agent tools |
| Structured output | `serde` deserialization from run output |
| Multi-agent | Graph-based orchestration with `GraphSpec` |
| Verifier | Primary and verifier pattern |
| Human-in-the-loop | Suspend and resume a run |
| Streaming | `Run` event iterator |
| Memory and RAG | Context injection and LanceDB retrieval |
| Providers | Ollama, Anthropic, OpenAI, Gemini |
| Chinese providers | GLM, Qwen, DeepSeek |
| Vector stores | LanceDB, pgvector, Milvus, Qdrant |
| Durability | `JournalStore`, `MemoryStore`, replay |
| Observability | Cost tracking and OTEL export |
| Policy | Data residency rules |
| MCP and A2A | Interoperability |
| Edge deployment | `cargo build --release`, static linking |
| Testing | Offline `tokio::test` patterns |
| Troubleshooting | Common errors and fixes |
| API reference | Full rustdoc symbol index |

## Requirements

- Rust 1.75 or later (2021 edition)
- Add to `Cargo.toml`:
  ```toml
  [dependencies]
  ancora-core = { git = "https://github.com/ancora-ai/ancora" }
  ancora-proto = { git = "https://github.com/ancora-ai/ancora" }
  ```

## Key types

| Type | Crate | Description |
|------|-------|-------------|
| `Run` | `ancora-core` | In-flight agent run |
| `RunStatus` | `ancora-core` | Terminal status of a run |
| `AgentSpec` | `ancora-core` | Model, instructions, tools, policy |
| `ToolSpec` | `ancora-core` | Tool name, description, schema |
| `EffectClass` | `ancora-core` | `Read`, `Write`, `None` |
| `MemoryStore` | `ancora-core` | In-memory journal (for tests) |
| `JournalStore` | `ancora-core` | Persistent journal trait |

See [Install](install.md) for full setup.
