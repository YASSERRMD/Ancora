# TypeScript SDK

The Ancora TypeScript SDK provides a type-safe API for building agents in
Node.js or Deno. It uses N-API bindings to call the native Ancora engine.
Structured output is validated with Zod, and all async operations return
native Promises or `AsyncIterable` streams.

## Pages

| Page | Description |
|------|-------------|
| [Install](install.md) | npm / Deno install and native build |
| [Quickstart](quickstart.md) | Minimal single-agent example |
| [Tools](tools.md) | Registering typed functions with Zod schemas |
| [Structured output](structured-output.md) | Zod schema output validation |
| [Multi-agent](multi-agent.md) | Graph-based orchestration |
| [Verifier](verifier.md) | Primary and verifier consensus |
| [Human-in-the-loop](human-in-the-loop.md) | Suspend and resume a run |
| [Streaming](streaming.md) | AsyncIterable event stream |
| [Memory and RAG](memory-and-rag.md) | Context injection and retrieval |
| [Providers](providers.md) | Ollama, Anthropic, OpenAI, Azure |
| [Chinese providers](chinese-providers.md) | GLM, Qwen, DeepSeek |
| [Vector stores](vector-stores.md) | LanceDB, pgvector, Milvus, Qdrant |
| [Durability](durability.md) | Persistent SQLite journal |
| [Observability](observability.md) | Cost tracking and OTEL export |
| [Policy](policy.md) | Data residency rules |
| [MCP and A2A](mcp-and-a2a.md) | Interoperability |
| [Edge deployment](edge-deployment.md) | WASM and single-binary edge deployment |
| [Testing](testing.md) | Offline test patterns with Vitest |
| [Troubleshooting](troubleshooting.md) | Common errors and fixes |
| [API reference](api-reference.md) | Full symbol index |
| [Migration](migration.md) | From Mastra and Vercel AI SDK |

## Requirements

- Node.js 20 or later (or Deno 2)
- A Rust toolchain to build the native N-API addon (or a pre-built binary)
- `npm install ancora`

## Schema convention

TypeScript agent specs use `instructions:` (not `systemPrompt:`):

```ts
import { buildSpec } from 'ancora'

const spec = buildSpec({
  model: 'llama3',
  instructions: 'You are a helpful assistant.',
})
```

See [Install](install.md) for full setup.
