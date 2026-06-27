# Using Chinese Providers and Self-Hosting

Ancora supports GLM, Qwen, DeepSeek, Kimi, and MiniMax with first-class API
and self-hosted configurations.

## GLM (Zhipu AI)

```bash
export GLM_API_KEY=...
```

```go
models := []string{"glm-4", "glm-4-flash", "glm-4-air", "glm-3-turbo"}
for _, model := range models {
    spec := ancora.NewAgentSpec(model, "Respond briefly.")
    handle, _ := agent.Run(spec)
    handle.CollectAll()
}
```

### Self-hosted GLM via llama.cpp

```bash
ANCORA_MODEL_URL=http://127.0.0.1:8080
```

```go
spec := ancora.NewAgentSpec("glm-4", "Respond.")
```

## Qwen (Alibaba Cloud)

```bash
export DASHSCOPE_API_KEY=...
# or for international:
export DASHSCOPE_API_URL=https://dashscope-intl.aliyuncs.com/compatible-mode/v1
```

```go
spec := ancora.NewAgentSpec("qwen-plus", "Respond.")
```

## DeepSeek

```bash
export DEEPSEEK_API_KEY=...
```

```go
spec := ancora.NewAgentSpec("deepseek-chat", "Respond.")
```

## Data residency note

These providers have data centers in China. If your data must stay in a
specific region, use the self-hosted GGUF variant or configure a regional
endpoint.

## See also

- [Providers](providers.md)
- [Policy](policy.md)
