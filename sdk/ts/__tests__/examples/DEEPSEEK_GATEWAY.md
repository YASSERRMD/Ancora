# deepseek-gateway

Demonstrates selecting DeepSeek model variants (deepseek-chat, deepseek-coder,
deepseek-reasoner) by model name in `buildSpec` and running each through the
standard agent transport.
Runs fully offline -- no DeepSeek API key required.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/deepseek-gateway-example
```

## What it shows

- Iterating over multiple DeepSeek model IDs
- Running each variant with the same offline runtime
- Verifying each run produces a distinct run ID
