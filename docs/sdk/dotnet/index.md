# .NET SDK

The Ancora .NET SDK targets .NET 8+ and uses P/Invoke to call the native
Ancora engine. Events are delivered via `IAsyncEnumerable<RunEvent>` and
structured output uses `System.Text.Json`.

## Pages

| Page | Description |
|------|-------------|
| [Install](install.md) | NuGet package and native library setup |
| [Quickstart](quickstart.md) | Minimal single-agent example |
| [Tools](tools.md) | Registering delegates as agent tools |
| [Structured output](structured-output.md) | System.Text.Json schema generation |
| [Multi-agent](multi-agent.md) | Graph-based orchestration |
| [Verifier](verifier.md) | Primary and verifier pattern |
| [Human-in-the-loop](human-in-the-loop.md) | Suspend and resume a run |
| [Streaming](streaming.md) | `IAsyncEnumerable<RunEvent>` iteration |
| [Memory and RAG](memory-and-rag.md) | Context injection and retrieval |
| [Providers](providers.md) | Ollama, Anthropic, OpenAI, Azure |
| [GLM self-host](glm-selfhost.md) | Local and self-hosted GLM models |
| [Vector stores](vector-stores.md) | pgvector, Milvus, Qdrant, Azure AI Search |
| [Durability](durability.md) | Persistent SQLite journal |
| [Observability](observability.md) | Cost tracking and OTEL export |
| [Policy](policy.md) | Data residency rules |
| [MCP and A2A](mcp-and-a2a.md) | Interoperability |
| [Deployment](deployment.md) | Single-binary, Docker, ASP.NET Core |
| [Testing](testing.md) | Offline xUnit test patterns |
| [Troubleshooting](troubleshooting.md) | Common errors and fixes |
| [API reference](api-reference.md) | Full symbol index |
| [Migration](migration.md) | From Microsoft.Extensions.AI and Semantic Kernel |

## Requirements

- .NET 8 SDK or later
- A Rust toolchain to build the native library (or a pre-built binary)
- `dotnet add package Ancora`

## Key types

| Type | Description |
|------|-------------|
| `Runtime` | Engine handle; dispose when done |
| `Agent (IDisposable)` | Runs agent tasks |
| `AgentSpec` | Model, instructions, tools, policy |
| `RunHandle` | Handle to an in-flight run |
| `RunEvent` | Base for `StartedEvent`, `TokenEvent`, `CompletedEvent`, `ResumedEvent`, `ToolCallEvent` |
| `ToolSpec` | Tool name, description, input schema |
| `ToolInputSchema` | JSON Schema for tool inputs |

Tests skip automatically when the native library is absent via
`DllNotFoundException` catch pattern.

See [Install](install.md) for full setup.
