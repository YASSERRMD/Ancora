# Ancora Message Schema

Source of truth: `crates/ancora-proto/proto/messages.proto`

## Role

```
ROLE_UNSPECIFIED = 0  (default, never sent)
ROLE_SYSTEM      = 1  (system prompt)
ROLE_USER        = 2  (human turn)
ROLE_ASSISTANT   = 3  (model turn)
ROLE_TOOL        = 4  (tool result injection)
```

## Content blocks

A `Message.content` field is a sequence of `ContentBlock` values. Each
block is a `oneof` selecting exactly one variant:

| Variant | Proto type | Description |
|---------|-----------|-------------|
| `text` | `TextContent` | Plain UTF-8 text |
| `tool_call` | `ToolCallContent` | Model requests a tool invocation |
| `tool_result` | `ToolResultContent` | Result of a tool invocation |
| `image` | `ImageContent` | Still image (inline base64 or URL) |
| `audio` | `AudioContent` | Audio clip (inline base64 or URL) |
| `document` | `DocumentContent` | Document file (inline base64 or URL) |

### TextContent

```proto
message TextContent {
  string text = 1;
}
```

### ToolCallContent

```proto
message ToolCallContent {
  string tool_call_id  = 1;  // stable within a run
  string tool_name     = 2;
  string arguments_json = 3; // JSON-encoded args
}
```

### ToolResultContent

```proto
message ToolResultContent {
  string tool_call_id  = 1;
  string result_json   = 2; // JSON-encoded result
  bool   is_error      = 3;
}
```

### ImageContent / AudioContent / DocumentContent

Each has a `oneof source` with `inline_base64` (raw bytes as base64
string with MIME prefix) or `url` (publicly accessible URL). `media_type`
carries the MIME type string (e.g. `image/png`, `audio/wav`,
`application/pdf`). `DocumentContent` also carries a `filename` hint.

## Message

```proto
message Message {
  string           id           = 1;
  Role             role         = 2;
  repeated ContentBlock content = 3;
  int64            created_at_ns = 4; // Unix epoch, nanoseconds
  TokenUsage       usage        = 5;
  Cost             cost         = 6;
  string           model_id     = 7;
}
```

## TokenUsage

```proto
message TokenUsage {
  uint64 input_tokens        = 1;
  uint64 output_tokens       = 2;
  uint64 cache_read_tokens   = 3;
  uint64 cache_write_tokens  = 4;
}
```

## Cost

Cost amounts are stored as **micro-cents** (millionths of a US cent) to
avoid floating-point in the wire format.

```proto
message Cost {
  uint64 input_micro_cents        = 1;
  uint64 output_micro_cents       = 2;
  uint64 cache_read_micro_cents   = 3;
  uint64 cache_write_micro_cents  = 4;
}
```

## Serialization

Canonical encoding is protobuf (binary). Every type also serializes to
JSON via serde (added in Phase 7). Protobuf and JSON representations are
kept equivalent by a round-trip CI test.
