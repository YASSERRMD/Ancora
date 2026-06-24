# Ancora JSON Schema

This directory contains JSON Schema 2020-12 documents for the principal
Ancora wire types. They mirror the protobuf definitions and are kept
in sync by the `gen-schema` binary.

## Files

| File | Type | Description |
|------|------|-------------|
| `Message.json` | Message | A single conversation turn |
| `ContentBlock.json` | ContentBlock | One content block (text/tool/image/etc) |
| `AgentSpec.json` | AgentSpec | Agent configuration |
| `ToolSpec.json` | ToolSpec | Tool contract |
| `JournalEvent.json` | JournalEvent | Durable journal entry envelope |
| `TokenUsage.json` | TokenUsage | Token counts per message |
| `Cost.json` | Cost | Cost in micro-cents per message |

## Regenerating

```bash
cargo run -p ancora-proto --bin gen-schema -- spec/schema
```

## Proto-JSON equivalence

The canonical format is protobuf. JSON is a mirror using the
proto3 JSON mapping (camelCase field names, enum string values).
Any field present in proto must round-trip through JSON without loss.
This is verified by CI tests in `ancora-proto`.
