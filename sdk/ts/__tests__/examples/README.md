# Ancora TypeScript SDK Examples

Example smoke tests demonstrating common patterns for the Ancora TypeScript SDK.
All examples run fully offline against a mock runtime -- no live API keys required.

## Run all example tests

```bash
cd sdk/ts
npx jest __tests__/examples
```

## Examples

| Example | Test file | Description |
|---------|-----------|-------------|
| `single-agent` | `single-agent-example.test.ts` | Start a run, collect events, read token text |
| `streaming-chat` | `streaming-chat-example.test.ts` | Stream tokens one-by-one via `for await` |
| `structured-output` | `structured-output-example.test.ts` | Derive JSON Schema from Zod model and inject into prompt |
| `schema-gen` | `schema-gen-example.test.ts` | Convert Zod schemas to Ancora tool input schemas |
| `mcp-tool-use` | `mcp-tool-use-example.test.ts` | Register and dispatch tools via ToolBridge |
| `multi-agent-verifier` | `multi-agent-verifier-example.test.ts` | Run primary and verifier agents concurrently |
| `human-in-loop` | `human-in-loop-example.test.ts` | Pause a run and resume with a decision |
| `rag-qdrant` | `rag-qdrant-example.test.ts` | Offline keyword retrieval injected as RAG context |
| `deepseek-gateway` | `deepseek-gateway-example.test.ts` | Configure DeepSeek model variants by model ID |
| `durable-restart` | `durable-restart-example.test.ts` | Persist events to journal and replay on restart |
| `cost-otel` | `cost-otel-example.test.ts` | Track spans, token estimates, and duration |

## Per-example READMEs

Each example has a companion Markdown file in this directory with a description,
test command, and a "What it shows" section.
