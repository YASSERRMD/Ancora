# Bedrock, Mistral, and Cohere Adapters

This guide explains how the three providers added in Phase 109 differ from
OpenAI-compatible providers and how they integrate with `ancora-inference`.

---

## AWS Bedrock

### Overview

Bedrock is a managed inference service that proxies multiple model families
through a unified AWS endpoint. Unlike a plain HTTP provider, Bedrock:

1. Embeds the **model ID in the URL path** rather than the request body.
2. Requires **AWS SigV4** request signing instead of a Bearer token.
3. Exposes separate endpoints for synchronous (`/invoke`) and streaming
   (`/invoke-with-response-stream`) calls.

### URL pattern

```
https://bedrock-runtime.<region>.amazonaws.com/model/<model-id>/invoke
https://bedrock-runtime.<region>.amazonaws.com/model/<model-id>/invoke-with-response-stream
```

The region is read from `AWS_REGION` (fallback: `AWS_DEFAULT_REGION`; default: `us-east-1`).

### Authentication

Bedrock uses AWS SigV4. The adapter reads three environment variables:

| Variable | Required | Purpose |
|---|---|---|
| `AWS_ACCESS_KEY_ID` | Yes | IAM access key |
| `AWS_SECRET_ACCESS_KEY` | Yes | IAM secret key |
| `AWS_SESSION_TOKEN` | No | Temporary session token (STS / instance profiles) |

The `sigv4_headers_stub` in `adapters/bedrock.rs` produces the correct
header set (`x-amz-date`, `x-amz-content-sha256`, `Authorization`,
and optionally `x-amz-security-token`). Replace the stub with a real
HMAC-SHA256 implementation (e.g. the `aws-sigv4` crate) for production use.

### Supported models

| Alias | Resolved model ID | Context | Tools | Vision |
|---|---|---|---|---|
| `claude-3-5-sonnet` | `anthropic.claude-3-5-sonnet-20241022-v2:0` | 200k | Yes | Yes |
| `claude-haiku` | `anthropic.claude-3-haiku-20240307-v1:0` | 200k | Yes | Yes |
| `llama3-70b` | `meta.llama3-70b-instruct-v1:0` | 8k | No | No |
| `mistral-large` | `mistral.mistral-large-2402-v1:0` | 32k | Yes | No |

### Wire format

Each Bedrock model family uses its own wire format (Claude uses the Anthropic
Messages API format, Llama uses a different schema). The current adapter stub
provides the URL-building and signing layer; a full implementation would
delegate to the model-family-appropriate wire encoder.

---

## Mistral AI

### Overview

Mistral's public API is OpenAI-compatible. The `OpenAiClient` adapter works
without modification -- only a different `ProviderProfile` is needed.

### Differences from OpenAI

| Feature | OpenAI | Mistral |
|---|---|---|
| Base URL | `api.openai.com` | `api.mistral.ai` |
| Auth | `Authorization: Bearer $OPENAI_API_KEY` | `Authorization: Bearer $MISTRAL_API_KEY` |
| Chat path | `/v1/chat/completions` | `/v1/chat/completions` (same) |
| SSE format | `data: {...}` with `choices[].delta.content` | identical |
| Tool calls | `tool_calls` in message | identical |

Mistral is a true drop-in replacement for OpenAI in the wire layer.

### Models

| Alias | Resolved ID | Context | Input $/M | Output $/M | Tools |
|---|---|---|---|---|---|
| `mistral-large` | `mistral-large-latest` | 128k | $2.00 | $6.00 | Yes |
| `mistral-small` | `mistral-small-latest` | 32k | $0.20 | $0.60 | Yes |
| `codestral` | `codestral-latest` | 32k | $0.20 | $0.60 | Yes |
| -- | `open-mistral-7b` | 32k | $0.25 | $0.25 | No |

---

## Cohere

### Overview

Cohere's Chat API uses a completely different wire format from OpenAI.
`CohereClient` in `adapters/cohere.rs` handles the conversion.

### Wire format comparison

| Concept | OpenAI | Cohere |
|---|---|---|
| Conversation | `messages[]` with role/content | `message` (current) + `chat_history[]` |
| System prompt | `{role: "system", content: "..."}` in messages | Top-level `preamble` field |
| User role | `"user"` | `"USER"` (uppercase) |
| Assistant role | `"assistant"` | `"CHATBOT"` (uppercase) |
| Tool definitions | `{type, function: {name, desc, parameters}}` (JSON Schema) | `{name, desc, parameter_definitions}` (flat map) |
| Streaming event | `data: {choices[].delta.content}` | `data: {event_type, text}` |
| Stream end | `data: [DONE]` | `data: {event_type: "stream-end", finish_reason}` |

### Conversation splitting

The current turn's message goes in the top-level `message` field. All prior
turns (excluding system messages) go into `chat_history`:

```json
{
  "model": "command-r-plus",
  "message": "What is the weather in Paris?",
  "preamble": "You are a helpful travel assistant.",
  "chat_history": [
    {"role": "USER", "message": "Hello"},
    {"role": "CHATBOT", "message": "Hi! How can I help?"}
  ]
}
```

### Tool parameter format

Cohere uses a flat `parameter_definitions` map instead of a JSON Schema object:

```json
{
  "name": "get_weather",
  "description": "Get current weather",
  "parameter_definitions": {
    "location": {
      "type": "str",
      "description": "City name",
      "required": true
    }
  }
}
```

### Models

| Alias | Resolved ID | Context | Input $/M | Output $/M |
|---|---|---|---|---|
| `command-r-plus-latest` | `command-r-plus` | 128k | $2.50 | $10.00 |
| `command-r-latest` | `command-r` | 128k | $0.15 | $0.60 |
| -- | `command` | 4k | $1.00 | $2.00 |

### Authentication

Cohere uses a Bearer token from `CO_API_KEY` (not `COHERE_API_KEY`):

```
Authorization: Bearer $CO_API_KEY
```

---

## Summary comparison

| Provider | Auth | URL pattern | Wire format | Streaming |
|---|---|---|---|---|
| Bedrock | AWS SigV4 | model in path | Model-family-specific | SSE chunks per family |
| Mistral | Bearer token | OpenAI path | OpenAI-identical | OpenAI SSE |
| Cohere | Bearer token | `/v1/chat` | Cohere Chat API | `event_type` SSE |
