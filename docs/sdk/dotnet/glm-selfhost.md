# GLM Self-Host (.NET)

Run a GLM model on your own hardware and connect Ancora to it.

## Option 1: Ollama with GLM weights

Ollama supports GLM via the `gguf` format:

```bash
ollama pull glm4:9b
```

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"
```

```csharp
var spec = new AgentSpec { Model = "glm4:9b", Instructions = "Answer in Chinese." };
```

## Option 2: Zhipu AI Cloud API

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

```csharp
var spec = new AgentSpec { Model = "glm-4-flash", Instructions = "Answer." };
```

## Option 3: LlamaEdge WASM runtime

LlamaEdge runs GLM as a WASM module with no GPU required:

```bash
# Install WasmEdge runtime
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

# Download GLM WASM model
wasmedge --dir .:. llama-api-server.wasm --model-name glm-4
```

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:8080/v1"
```

```csharp
var spec = new AgentSpec { Model = "glm-4", Instructions = "Answer." };
```

## See also

- [Providers](providers.md)
- [Vector stores](vector-stores.md)
