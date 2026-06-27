# Qwen Regional Endpoints (Java)

## Alibaba Cloud (global)

```bash
export ANCORA_MODEL_URL="https://dashscope.aliyuncs.com/compatible-mode/v1"
export DASHSCOPE_API_KEY="sk-..."
```

```java
var spec = new AgentSpec("qwen-plus", "Answer.", List.of(), 4096, 0.7f);
```

Available models:

| Model | Notes |
|-------|-------|
| `qwen-turbo` | Fast, low-cost |
| `qwen-plus` | Balanced quality |
| `qwen-max` | Highest quality |
| `qwen-long` | Extra-long context |

## Singapore regional endpoint

```bash
export ANCORA_MODEL_URL="https://dashscope-intl.aliyuncs.com/compatible-mode/v1"
```

## Switching endpoints at runtime

```java
// Read from environment to support multiple deployments
String endpoint = System.getenv().getOrDefault(
    "DASHSCOPE_ENDPOINT",
    "https://dashscope.aliyuncs.com/compatible-mode/v1"
);

// Set via system property for container deployments
System.setProperty("ancora.modelUrl", endpoint);
```

## DeepSeek

```bash
export ANCORA_MODEL_URL="https://api.deepseek.com/v1"
export DEEPSEEK_API_KEY="sk-..."
```

```java
var spec = new AgentSpec("deepseek-chat", "Answer.", List.of(), 4096, 0.7f);
```

## GLM (Zhipu AI)

```bash
export ANCORA_MODEL_URL="https://open.bigmodel.cn/api/paas/v4"
export GLM_API_KEY="your-key"
```

```java
var spec = new AgentSpec("glm-4-flash", "Answer in Chinese.", List.of(), 4096, 0.7f);
```

## See also

- [Providers](providers.md)
- [Vector stores](vector-stores.md)
