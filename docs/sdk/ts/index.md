# TypeScript SDK

The Ancora TypeScript SDK provides a type-safe API for building agents in
Node.js or Deno. It uses N-API bindings to call the native Ancora engine.

## Pages

| Page | Description |
|------|-------------|
| Install | npm / Deno install and native build |
| Quickstart | Minimal single-agent example |
| Tools | Registering typed functions as agent tools |
| Structured output | Zod schema output validation |
| Multi-agent | Graph-based orchestration |
| Verifier | Primary and verifier consensus |
| Human-in-the-loop | Suspend and resume a run |
| Streaming | AsyncIterable event stream |
| Memory and RAG | Context injection and retrieval |
| Providers | Ollama, Anthropic, OpenAI, Azure |
| Chinese providers | GLM, Qwen, DeepSeek |
| Vector stores | LanceDB, pgvector, Milvus, Qdrant |
| Durability | Persistent SQLite journal |
| Observability | Cost tracking and OTEL export |
| Policy | Data residency rules |
| MCP and A2A | Interoperability |
| Edge deployment | Bundled binary for Electron or container |
| Testing | Offline test patterns with Vitest |
| Troubleshooting | Common errors and fixes |
| API reference | Full symbol index |

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
