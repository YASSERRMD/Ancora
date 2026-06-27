# qwen_gateway

Configures agent specs for the Qwen model family (qwen-turbo, qwen-plus,
qwen-max, qwen-long) and runs each variant through the standard agent
transport, demonstrating model selection by model ID.
Runs fully offline -- no DashScope API key required.

## Run

```bash
cd sdk/python
python -m examples.qwen_gateway
```

## What it shows

- Selecting Qwen model variants by `model_id` in `AgentSpec`
- Running multiple agent specs with the same `Runtime`
- Provider routing by model name at the runtime level
