# Local inference with OpenAI-compatible endpoints

`OpenAiClient` points at any server that speaks the OpenAI chat-completions API.
Set `base_url` to the server root; the client appends `/v1/chat/completions` automatically.

## Ollama

Start Ollama and pull a model:

```
ollama serve
ollama pull llama3
```

Create the client:

```rust
use ancora_inference::openai::OpenAiClient;

let client = OpenAiClient::new("http://localhost:11434");
```

Pass `model_id: "llama3"` (or any pulled model) in `CompletionRequest`.

## vLLM

Start vLLM with an OpenAI-compatible server:

```
python -m vllm.entrypoints.openai.api_server --model mistralai/Mistral-7B-v0.1
```

Create the client:

```rust
let client = OpenAiClient::new("http://localhost:8000");
```

## llama.cpp server

```
./server -m model.gguf -c 2048
```

```rust
let client = OpenAiClient::new("http://localhost:8080");
```

## OpenAI

```rust
let client = OpenAiClient::new("https://api.openai.com");
```

Set the `Authorization: Bearer <key>` header at the HTTP layer or wrap
`OpenAiClient` in your own adapter that injects the key before calling `post`.
