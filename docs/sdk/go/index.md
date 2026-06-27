# Go SDK

The Ancora Go SDK (`ancora.io/sdk`) wraps the Rust engine via CGo and
provides idiomatic Go types for building, running, and observing agents.

## Contents

| Page | Description |
|------|-------------|
| [Install](install.md) | Prerequisites and Go module setup |
| [Quickstart](quickstart.md) | Single agent in under 10 lines |
| [Defining tools](tools.md) | GoToolRegistry and tool registration |
| [Structured output](structured-output.md) | SchemaFromStruct and typed results |
| [Multi-agent graph](multi-agent.md) | Graph spec and concurrent runs |
| [Verifier pattern](verifier.md) | Primary + verifier consensus |
| [Human-in-the-loop](human-in-the-loop.md) | Pause and resume |
| [Streaming](streaming.md) | EventChan and token accumulation |
| [Memory and RAG](memory-and-rag.md) | LanceDB and vector retrieval |
| [Choosing a provider](providers.md) | Ollama, Anthropic, OpenAI, etc. |
| [Chinese providers](chinese-providers.md) | GLM, Qwen, DeepSeek |
| [Vector stores](vector-stores.md) | Store selection and configuration |
| [Durability](durability.md) | StoringTransport and SqliteStore |
| [Observability](observability.md) | Cost tracking and OTEL spans |
| [Policy](policy.md) | Data residency rules |
| [MCP and A2A](mcp-and-a2a.md) | Tool and agent interop |
| [Edge deployment](edge-deployment.md) | Single-binary, CGo, air-gapped |
| [Testing](testing.md) | Offline tests and fixtures |
| [Troubleshooting](troubleshooting.md) | Common errors and fixes |
| [API reference](api-reference.md) | Package-level symbol index |
