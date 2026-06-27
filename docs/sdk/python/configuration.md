# Configuration (Python)

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ANCORA_MODEL_URL` | `http://127.0.0.1:11434` | Inference endpoint URL |
| `ANCORA_LOG_LEVEL` | `warn` | Log level: `trace`, `debug`, `info`, `warn`, `error` |
| `ANCORA_JOURNAL_PATH` | (none) | Default SQLite journal path; overrides per-`Runtime` setting |
| `ANTHROPIC_API_KEY` | (none) | API key for Anthropic endpoints |
| `OPENAI_API_KEY` | (none) | API key for OpenAI endpoints |
| `GEMINI_API_KEY` | (none) | API key for Google Gemini |
| `GLM_API_KEY` | (none) | API key for Zhipu GLM |
| `DASHSCOPE_API_KEY` | (none) | API key for Alibaba Qwen |
| `DEEPSEEK_API_KEY` | (none) | API key for DeepSeek |

## Runtime configuration

```python
from ancora import Runtime, SqliteStore, StoringTransport

rt = Runtime(
    transport=StoringTransport(SqliteStore("/var/lib/myapp/journal.db")),
    log_level="info",
)
```

## Per-run configuration

```python
from ancora import AgentSpec, PolicySpec

spec = AgentSpec(
    model="llama3",
    instructions="Answer.",
    max_tokens=2048,
    temperature=0.3,
    model_url="http://127.0.0.1:11434",   # overrides ANCORA_MODEL_URL
    policy=PolicySpec(
        allow_regions=["us-east-1"],
        max_write_tools=2,
    ),
)
```

## Logging

```bash
ANCORA_LOG_LEVEL=debug python agent.py
```

Ancora uses the Rust `tracing` crate internally. Debug level shows every
activity recorded and replayed. Trace level shows FFI boundary crossings.

## dotenv support

Ancora does not load `.env` files automatically. Use `python-dotenv`:

```bash
pip install python-dotenv
```

```python
from dotenv import load_dotenv
load_dotenv()

from ancora import Runtime
rt = Runtime()
```

## See also

- [Install](install.md)
- [Providers](providers.md)
