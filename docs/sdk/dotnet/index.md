# .NET SDK

The Ancora .NET SDK targets .NET 8+ and uses P/Invoke to call the native
Ancora engine.

## Pages

| Page | Description |
|------|-------------|
| Install | NuGet package and native library setup |
| Quickstart | Minimal single-agent example |
| Tools | Registering delegates as agent tools |
| Structured output | System.Text.Json schema generation |
| Multi-agent | Graph-based orchestration |
| Verifier | Primary and verifier pattern |
| Human-in-the-loop | Suspend and resume a run |
| Streaming | `IAsyncEnumerable<RunEvent>` iteration |
| Memory and RAG | Context injection and retrieval |
| Providers | Ollama, Anthropic, OpenAI, Azure |
| Chinese providers | GLM, Qwen, DeepSeek |
| Vector stores | LanceDB, pgvector, Milvus, Qdrant |
| Durability | Persistent SQLite journal |
| Observability | Cost tracking and OTEL export |
| Policy | Data residency rules |
| MCP and A2A | Interoperability |
| Edge deployment | Single-binary deployment |
| Testing | Offline xUnit test patterns |
| Troubleshooting | Common errors and fixes |
| API reference | Full symbol index |

## Requirements

- .NET 8 SDK or later
- A Rust toolchain to build the native library (or a pre-built binary)
- `dotnet add package Ancora`

## Key types

| Type | Description |
|------|-------------|
| `Runtime` | Engine handle; dispose when done |
| `Agent(IDisposable)` | Runs agent tasks |
| `AgentSpec` | Model, instructions, tools, policy |
| `RunHandle` | Handle to an in-flight run |
| `RunEvent` | Base for `StartedEvent`, `TokenEvent`, `CompletedEvent`, `ResumedEvent`, `ToolCallEvent` |
| `ToolSpec` | Tool name, description, input schema |
| `ToolInputSchema` | JSON Schema for tool inputs |

Tests skip automatically when the native library is absent via
`DllNotFoundException` catch pattern.

See [Install](install.md) for full setup.
