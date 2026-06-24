# ancora (TypeScript SDK)

TypeScript/Node.js bindings for the Ancora agent runtime, built with [napi-rs](https://napi.rs/).

## Requirements

- Node.js >= 18
- Rust toolchain (for building the native addon)

## Build

```sh
npm install
npm run build
```

## Usage

```typescript
import { Runtime, version } from 'ancora'

const rt = new Runtime()

const runId = rt.startRun(JSON.stringify({ model: 'test', instructions: 'hello' }))

let event: string | null
while ((event = rt.pollRun(runId)) !== null) {
  const ev = JSON.parse(event)
  console.log(ev)
}

rt.free()
console.log('version:', version())
```

## Test

```sh
npm test
```

## License

Apache-2.0
