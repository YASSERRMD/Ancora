# Roadmap: Edge and Small-Model Support

## Edge deployment (v0.7.0)

- Plugin execution on edge nodes with limited connectivity
- Offline catalog cache for air-gapped environments
- Compressed model artifacts for constrained devices

## Small-model support (v0.7.0 - v0.8.0)

- Quantized inference backends (GGUF, GGML)
- Small-model routing: automatically select smaller models for low-complexity tasks
- Latency-aware scheduling for edge-hosted models

## Timeline

| Target | Feature |
|---|---|
| v0.7.0 | Edge node plugin runtime |
| v0.7.0 | Offline catalog cache |
| v0.7.0 | Quantized inference backend |
| v0.8.0 | Small-model router |
| v0.8.0 | Python FFI async generators |
