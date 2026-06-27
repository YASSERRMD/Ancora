# Configuration (Java)

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ANCORA_MODEL_URL` | `http://127.0.0.1:11434` | Inference endpoint URL |
| `ANCORA_LOG_LEVEL` | `warn` | Log level: `trace`, `debug`, `info`, `warn`, `error` |
| `ANTHROPIC_API_KEY` | (none) | API key for Anthropic endpoints |
| `OPENAI_API_KEY` | (none) | API key for OpenAI endpoints |
| `GLM_API_KEY` | (none) | API key for Zhipu GLM |
| `DASHSCOPE_API_KEY` | (none) | API key for Alibaba Qwen |
| `DEEPSEEK_API_KEY` | (none) | API key for DeepSeek |

## `RuntimeOptions`

```java
import io.ancora.*;

var store = new SqliteStore("/var/lib/myapp/journal.db");
var rt = new Runtime(new RuntimeOptions()
    .withTransport(new StoringTransport(store))
    .withLogLevel("info")
    .withHttpTimeout(Duration.ofMinutes(10))
);
```

## Reading from `application.properties` (Spring Boot)

```properties
ancora.model-url=http://127.0.0.1:11434
ancora.model=llama3
ancora.journal-path=/var/lib/myapp/journal.db
```

```java
@Value("${ancora.model}")
private String model;

@Value("${ancora.model-url}")
private String modelUrl;
```

## Reading from system properties

```java
String modelUrl = System.getProperty("ancora.modelUrl",
    System.getenv().getOrDefault("ANCORA_MODEL_URL", "http://127.0.0.1:11434"));
```

## Logging

```bash
ANCORA_LOG_LEVEL=debug java -jar myapp.jar
```

Ancora uses the Rust `tracing` crate internally. Debug level shows every
activity recorded and replayed.

## See also

- [Install](install.md)
- [Providers](providers.md)
