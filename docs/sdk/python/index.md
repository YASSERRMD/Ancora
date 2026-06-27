# Python SDK

The Ancora Python SDK wraps the native Ancora engine via CFFI bindings.
It supports synchronous and async usage, Pydantic structured output, decorator
tool registration, and the full durability and policy feature set.

## Pages

| Page | Description |
|------|-------------|
| [Install](install.md) | Install the Python package and build the native library |
| [Quickstart](quickstart.md) | Minimal single-agent example |
| [Tools](tools.md) | Registering Python callables as agent tools |
| [Structured output](structured-output.md) | Pydantic model output schemas |
| [Multi-agent](multi-agent.md) | Graph-based orchestration |
| [Verifier](verifier.md) | Primary and verifier consensus pattern |
| [Human-in-the-loop](human-in-the-loop.md) | Suspend and resume a run |
| [Streaming](streaming.md) | Token-by-token event iteration |
| [Memory and RAG](memory-and-rag.md) | Context injection and vector retrieval |
| [Providers](providers.md) | Switching between Ollama, Anthropic, OpenAI |
| [Chinese providers](chinese-providers.md) | GLM, Qwen, DeepSeek |
| [Vector stores](vector-stores.md) | LanceDB, pgvector, Milvus, Qdrant |
| [Durability](durability.md) | Persistent journal and replay |
| [Observability](observability.md) | Cost tracking and OTEL export |
| [Policy](policy.md) | Data residency rules |
| [MCP and A2A](mcp-and-a2a.md) | Interoperability |
| [Packaging and deployment](deployment.md) | Docker, air-gapped, Lambda |
| [Testing](testing.md) | Offline pytest patterns |
| [Troubleshooting](troubleshooting.md) | Common errors and fixes |
| [API reference](api-reference.md) | Full symbol index |
| [Migration](migration.md) | From LangChain, LangGraph, CrewAI |

## Requirements

- Python 3.10 or later
- A Rust toolchain to build the native library (or a pre-built binary)
- `pip install cffi` (installed automatically as a dependency)

## Quick install

```bash
pip install ancora
```

See [Install](install.md) for build-from-source instructions.
