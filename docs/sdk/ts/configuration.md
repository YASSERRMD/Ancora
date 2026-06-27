# Configuration (TypeScript)

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ANCORA_MODEL_URL` | `http://127.0.0.1:11434` | Inference endpoint URL |
| `ANCORA_LOG_LEVEL` | `warn` | Log level: `trace`, `debug`, `info`, `warn`, `error` |
| `ANTHROPIC_API_KEY` | (none) | API key for Anthropic endpoints |
| `OPENAI_API_KEY` | (none) | API key for OpenAI endpoints |
| `GEMINI_API_KEY` | (none) | API key for Google Gemini |
| `GLM_API_KEY` | (none) | API key for Zhipu GLM |
| `DASHSCOPE_API_KEY` | (none) | API key for Alibaba Qwen |
| `DEEPSEEK_API_KEY` | (none) | API key for DeepSeek |

## Runtime options

```ts
import { Runtime, SqliteStore, StoringTransport } from 'ancora'

const rt = new Runtime({
  transport: new StoringTransport(new SqliteStore('/var/lib/myapp/journal.db')),
  logLevel: 'info',
})
```

## dotenv support

```bash
npm install dotenv
```

```ts
import 'dotenv/config'
import { Runtime } from 'ancora'

const rt = new Runtime()   // picks up ANCORA_MODEL_URL from .env
```

## Per-spec model URL override

```ts
const spec = buildSpec({
  model: 'llama3',
  instructions: 'Answer.',
  modelUrl: 'http://127.0.0.1:11434',   // overrides ANCORA_MODEL_URL
})
```

## Logging

```bash
ANCORA_LOG_LEVEL=debug node agent.js
```

Ancora uses the Rust `tracing` crate internally. Debug level shows every
activity recorded and replayed.

## See also

- [Install](install.md)
- [Providers](providers.md)
