# Extension Points Reference

This document describes each stable extension point exposed by the `ancora-plugin` SDK.

## `provider` - LLM Provider

Implement the `ProviderPlugin` trait to integrate an external large-language-model API.

Key types:
- `GenerateRequest` - input messages, model name, sampling parameters
- `GenerateResponse` - generated content, token counts, truncation flag
- `ProviderError` - auth failures, rate limits, model-not-found, etc.

Required scope: `llm:generate` (recommended)

## `vector_store` - Vector Store

Implement the `VectorStorePlugin` trait to provide a vector similarity search back-end.

Key types:
- `Document` - id, content, embedding, metadata
- `QueryRequest` - query embedding, top-k, optional namespace
- `QueryResult` - matched document with cosine similarity score

## `tool` - Agent Tool

Implement the `ToolPlugin` trait to expose a callable function to agents.

Key types:
- `ToolSpec` - name, description, argument schema
- `ToolInput` - argument values
- `ToolOutput` - result value and optional summary

Required scope: `tool:execute` (recommended)

## `memory` - Memory Backend

Implement the `MemoryPlugin` trait to persist and retrieve agent memory.

Key types:
- `MemoryEntry` - key, value, tags, timestamp
- `MemoryQuery` - tag filter, key prefix, limit

## `guardrail` - Guardrail

Implement the `GuardrailPlugin` trait to intercept and filter agent inputs and outputs.

Key types:
- `GuardrailRequest` - content kind (input/output), text, context
- `GuardrailDecision` - Allow | Rewrite(String) | Block(String)

## `grader` - Response Grader

Implement the `GraderPlugin` trait to score agent responses.

Key types:
- `GradeRequest` - prompt, response, optional reference
- `Grade` - numeric score in [0.0, 1.0], pass/fail, rationale

Required scope: `grader:run` (recommended)

## `exporter` - Telemetry Exporter

Implement the `ExporterPlugin` trait to forward telemetry to an external sink.

Key types:
- `Span` - trace/span ids, name, timing, attributes, status
- `MetricPoint` - name, value, labels, timestamp
