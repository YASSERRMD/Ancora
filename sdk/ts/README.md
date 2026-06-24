# ancora (TypeScript SDK)

TypeScript/Node.js bindings for the Ancora agent runtime, built with [napi-rs](https://napi.rs/).

## Install

```sh
npm install ancora
```

Prebuilt native binaries are available for:

| Platform | Architecture |
|----------|-------------|
| Linux    | x64 (GNU)   |
| Linux    | arm64 (GNU) |
| macOS    | x64         |
| macOS    | arm64       |
| Windows  | x64 (MSVC)  |

The correct binary is installed automatically via `optionalDependencies`.

## Quickstart

```typescript
import { Agent, buildSpec, tokenText, collectEvents } from 'ancora'

const spec = buildSpec('claude-3-5-sonnet', {
  instructions: 'You are a helpful assistant.',
  maxTokens: 1024,
})

const agent = new Agent()
const handle = agent.run(spec)
const events = await collectEvents(handle)
console.log(tokenText(events))
agent.free()
```

## Quickstart with tools

```typescript
import { z } from 'zod'
import { Agent, buildSpec, defineTool, createToolBridge, tokenText, collectEvents } from 'ancora'

const weatherTool = defineTool({
  name: 'get_weather',
  description: 'Get the current weather for a city',
  schema: z.object({ city: z.string() }),
  handler: async ({ city }) => ({ temperature: '22C', city }),
})

const spec = buildSpec('claude-3-5-sonnet', {
  instructions: 'Use tools to answer questions.',
  maxTokens: 1024,
})

const agent = new Agent()
const handle = agent.run(spec)
const bridge = createToolBridge(weatherTool)

const events = await collectEvents(bridge.run(handle))
console.log(tokenText(events))
agent.free()
```

## Quickstart (browser / edge, WASM transport)

```typescript
import { WasmRuntime, buildSpec, tokenText, collectEvents } from 'ancora'

const rt = new WasmRuntime({ baseUrl: 'http://localhost:8080' })
const handle = await rt.run(buildSpec('claude-3-5-sonnet'))
const events = await collectEvents(handle)
console.log(tokenText(events))
```

## Build from source

```sh
npm install
npm run build       # release build
npm run build:debug # debug build
npm run build:wasm  # browser ESM bundle
```

Requires a Rust toolchain. The `build` script calls `napi build --platform --release`.

## Test

```sh
npm test
```

## Requirements

- Node.js >= 18
- Rust toolchain (only needed when building from source)

## License

Apache-2.0
