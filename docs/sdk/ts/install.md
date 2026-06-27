# Install (TypeScript)

## Requirements

- Node.js 20 or later (or Deno 2)
- A Rust toolchain to build the native N-API addon (or use a pre-built binary)

## Install from npm

```bash
npm install ancora
```

Pre-built N-API binaries are bundled for common platforms
(`linux-x64`, `linux-arm64`, `darwin-arm64`, `darwin-x64`, `win32-x64`).

## Build from source

```bash
# 1. Build the native Rust library
cargo build --release -p ancora-ffi

# 2. Set environment variables
export ANCORA_LIB_DIR="$(pwd)/target/release"
export ANCORA_INCLUDE_DIR="$(pwd)/crates/ancora-ffi/include"

# 3. Install in local mode
npm install sdk/ts
```

## TypeScript configuration

Ancora ships type declarations. Ensure your `tsconfig.json` targets ES2022+:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "strict": true
  }
}
```

## ESM and CommonJS

The package ships both ESM and CJS entry points:

```ts
// ESM
import { Runtime, buildSpec } from 'ancora'

// CJS
const { Runtime, buildSpec } = require('ancora')
```

## Runtime prerequisites

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"   # Ollama (default)
ollama pull llama3
```

## Verify

```ts
import { version } from 'ancora'
console.log(version)
```

## See also

- [Quickstart](quickstart.md)
- [Providers](providers.md)
