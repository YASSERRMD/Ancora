# Python SDK

The Ancora Python SDK wraps the native Ancora engine via CFFI bindings.

## Pages

| Page | Description |
|------|-------------|
| Install | Install the Python package and build the native library |
| Quickstart | Minimal single-agent example |
| Tools | Registering Python callables as agent tools |
| Structured output | Pydantic model output schemas |
| Multi-agent | Graph-based orchestration |
| Verifier | Primary and verifier pattern |
| Human-in-the-loop | Suspend and resume a run |
| Streaming | Token-by-token event iteration |
| Memory and RAG | Context injection and vector retrieval |
| Providers | Switching between Ollama, Anthropic, OpenAI |
| Chinese providers | GLM, Qwen, DeepSeek |
| Vector stores | LanceDB, pgvector, Milvus, Qdrant |
| Durability | Persistent journal and replay |
| Observability | Cost tracking and OTEL export |
| Policy | Data residency rules |
| MCP and A2A | Interoperability |
| Edge deployment | Embedded binary deployment |
| Testing | Offline test patterns |
| Troubleshooting | Common errors and fixes |
| API reference | Full symbol index |

## Requirements

- Python 3.10 or later
- A Rust toolchain to build the native library (or a pre-built binary)
- CFFI (`pip install cffi`)

## Quick install

```bash
pip install ancora
```

See [Install](install.md) for build-from-source instructions.
