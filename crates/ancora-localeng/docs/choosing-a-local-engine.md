# Choosing a Local Inference Engine

Use this guide to select the right engine for your deployment scenario.

## Decision Tree

```
Do you need GPU throughput for many concurrent requests?
  YES --> GPU available?
            YES --> Prefer vLLM or SGLang (continuous batching)
            NO  --> llama.cpp server (CPU optimised)
  NO  --> Do you need the simplest possible setup?
            YES --> Ollama (handles model downloads automatically)
            NO  --> Do you need in-process inference (no HTTP server)?
                      YES --> llama.cpp embedded (low latency, smallest footprint)
                      NO  --> LM Studio (GUI + API, good for prototyping)
```

## Engine Comparison

### llama.cpp server
- Best for: CPU inference on consumer hardware, GGUF models.
- Strengths: grammar-constrained decoding, quantisation, low memory.
- Limitations: single-user, limited batching.

### llama.cpp embedded
- Best for: edge deployments, CLI tools, smallest footprint.
- Strengths: no HTTP overhead, embeds directly in your binary.
- Limitations: no streaming, blocks the calling thread.

### Ollama
- Best for: developer laptops, quick prototyping.
- Strengths: automatic model management, multi-modal, function calling.
- Limitations: limited batch throughput.

### vLLM
- Best for: production GPU servers, high concurrency.
- Strengths: PagedAttention, continuous batching, speculative decoding.
- Limitations: requires GPU, heavier startup.

### SGLang
- Best for: structured generation, RadixAttention workloads.
- Strengths: grammar-constrained, LoRA, speculative decoding.
- Limitations: narrower model support than vLLM.

### LM Studio
- Best for: GUI-first workflows, non-technical users.
- Strengths: easy model browsing, OpenAI-compatible API.
- Limitations: desktop-only, no headless server mode.

### TGI (Text Generation Inference)
- Best for: Hugging Face model hub integration.
- Strengths: streaming, continuous batching, quantisation.
- Limitations: no grammar-constrained decoding.

### ONNX Runtime
- Best for: embeddings, vision, cross-platform edge inference.
- Strengths: CoreML/CUDA acceleration, wide model support.
- Limitations: no streaming text generation.

## Hardware Recommendations

| Available Hardware | Recommended Engine |
|---|---|
| CPU only, < 16 GB RAM | llama.cpp embedded |
| CPU only, >= 16 GB RAM | llama.cpp server |
| Apple Silicon (Metal) | Ollama |
| NVIDIA GPU, single user | Ollama or llama.cpp server |
| NVIDIA GPU, high concurrency | vLLM |
| NVIDIA GPU, structured gen | SGLang |
| Any, embeddings only | ONNX Runtime |
