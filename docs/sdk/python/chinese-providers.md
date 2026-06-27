# Chinese Providers (Python)

Ancora supports GLM (Zhipu AI), Qwen (Alibaba Cloud), and DeepSeek as
first-class inference providers.

## GLM (Zhipu AI)

```bash
export ANCORA_MODEL_URL="https://open.bigmodel.cn/api/paas/v4"
export GLM_API_KEY="your-key"
```

Available models:

| Model | Context | Notes |
|-------|---------|-------|
| `glm-4` | 128 k | Flagship reasoning model |
| `glm-4-flash` | 128 k | Low-latency variant |
| `glm-4-air` | 128 k | Cost-efficient |
| `glm-4-airx` | 8 k | Ultra-low latency |
| `glm-4v` | 8 k | Vision capable |

```python
spec = AgentSpec(model="glm-4-flash", instructions="Answer in Chinese.")
```

## Qwen (Alibaba Cloud)

```bash
export ANCORA_MODEL_URL="https://dashscope.aliyuncs.com/compatible-mode/v1"
export DASHSCOPE_API_KEY="sk-..."
```

Available models:

| Model | Notes |
|-------|-------|
| `qwen-turbo` | Fast, low-cost |
| `qwen-plus` | Balanced quality |
| `qwen-max` | Highest quality |
| `qwen-long` | Extra-long context |

```python
spec = AgentSpec(model="qwen-plus", instructions="Answer in English.")
```

## Qwen regional endpoints

Use `ANCORA_MODEL_URL` to target a regional endpoint:

```bash
# Singapore
export ANCORA_MODEL_URL="https://dashscope-intl.aliyuncs.com/compatible-mode/v1"
```

## DeepSeek

```bash
export ANCORA_MODEL_URL="https://api.deepseek.com/v1"
export DEEPSEEK_API_KEY="sk-..."
```

```python
spec = AgentSpec(model="deepseek-chat", instructions="Answer concisely.")
```

## See also

- [Providers](providers.md)
- [Vector stores](vector-stores.md)
