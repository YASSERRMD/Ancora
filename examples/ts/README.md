# Ancora TypeScript Examples

Runnable examples for the Ancora TypeScript SDK. All examples use an offline
runtime and do not require network access or API keys.

## Examples

| File | Description |
|------|-------------|
| `single-agent.ts` | Simple single-agent run using the Agent class |
| `streaming-chat.ts` | Token-by-token streaming output via `for await` |
| `mcp-tool-use.ts` | Tool dispatch with ToolBridge and multiple tools |
| `run-once.ts` | Convenience wrapper for a single-shot agent run |
| `validate-spec.ts` | Spec validation with `validateSpec` and `buildSpec` |
| `event-loop.ts` | Explicit `switch` over event kinds |
| `multi-tool-chain.ts` | Chaining multiple tools with `createToolBridge` |
| `schema-gen.ts` | Generate JSON Schema from Zod schemas with `zodToInputSchema` |
| `error-handling.ts` | Tool error recovery with ToolBridge |

## Run an example

Install dependencies (uses the SDK from the local workspace):

```sh
npm install
```

Run any example with ts-node:

```sh
npm run run:single-agent
npm run run:streaming-chat
npm run run:mcp-tool-use
```

## Offline mode

All examples use an in-memory offline runtime. No native addon build is needed.
The `offline-runtime.ts` helper provides a mock `Runtime` that produces
deterministic events.
