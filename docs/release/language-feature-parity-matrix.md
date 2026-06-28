# Language Feature Parity Matrix

## Ancora 0.6.0

| Feature | Rust | Go | Python | TypeScript | .NET | Java |
|---------|------|----|--------|------------|------|------|
| Single-agent run | yes | yes | yes | yes | yes | yes |
| Multi-agent (parallel) | yes | yes | yes | yes | yes | yes |
| Verifier agent | yes | yes | yes | yes | yes | yes |
| Human-in-the-loop | yes | yes | yes | yes | yes | yes |
| Streaming tokens | yes | yes | yes | yes | yes | yes |
| Structured output | yes | yes | yes | yes | yes | yes |
| Memory and RAG | yes | yes | yes | yes | yes | yes |
| MCP tool use | yes | yes | yes | yes | yes | yes |
| A2A handoff | yes | yes | yes | yes | yes | yes |
| OTel tracing | yes | yes | yes | yes | yes | yes |
| Cost accounting | yes | yes | yes | yes | yes | yes |
| Durability replay | yes | yes | yes | yes | yes | yes |
| Policy enforcement | yes | yes | yes | yes | yes | yes |
| Edge/WASM | yes | no | no | yes | no | no |
| Chinese providers | yes | yes | yes | yes | yes (GLM) | yes (Qwen) |
| Local-only mode | yes | yes | yes | yes | yes | yes |
| Journal interop | yes | yes | yes | yes | yes | yes |
| Error handling kinds | yes | yes | yes | yes | yes | yes |

## Notes

- "Edge/WASM" requires `wasm32-unknown-unknown` target; only Rust and TypeScript support it.
- "Chinese providers" column shows the primary provider tested per language; all languages support the full list via HTTP.
- All features are tested offline via recorded fixtures or in-process mocks.
