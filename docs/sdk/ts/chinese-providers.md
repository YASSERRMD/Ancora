# Chinese Providers (TypeScript)

## DeepSeek via gateway

```bash
export ANCORA_MODEL_URL="https://api.deepseek.com/v1"
export DEEPSEEK_API_KEY="sk-..."
```

```ts
const spec = buildSpec({ model: 'deepseek-chat', instructions: 'Answer concisely.' })
```

Available models:

| Model | Notes |
|-------|-------|
| `deepseek-chat` | General purpose |
| `deepseek-coder` | Code-focused |
| `deepseek-reasoner` | Extended thinking |

## GLM (Zhipu AI)

```bash
export ANCORA_MODEL_URL="https://open.bigmodel.cn/api/paas/v4"
export GLM_API_KEY="your-key"
```

```ts
const spec = buildSpec({ model: 'glm-4-flash', instructions: 'Answer in Chinese.' })
```

| Model | Context | Notes |
|-------|---------|-------|
| `glm-4` | 128 k | Flagship |
| `glm-4-flash` | 128 k | Low-latency |
| `glm-4-air` | 128 k | Cost-efficient |
| `glm-4v` | 8 k | Vision |

## Qwen (Alibaba Cloud)

```bash
export ANCORA_MODEL_URL="https://dashscope.aliyuncs.com/compatible-mode/v1"
export DASHSCOPE_API_KEY="sk-..."
```

```ts
const spec = buildSpec({ model: 'qwen-plus', instructions: 'Answer.' })
```

## Qwen regional endpoint

```bash
export ANCORA_MODEL_URL="https://dashscope-intl.aliyuncs.com/compatible-mode/v1"
```

## See also

- [Providers](providers.md)
- [Vector stores](vector-stores.md)
