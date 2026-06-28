# Per-language Usage Notes

Ancora's canonical implementation is in Rust. Notes below apply when wrapping or
porting to another language.

## Rust (canonical)

Add the desired crates to `Cargo.toml`:

```toml
ancora-orchestrate = { path = "crates/ancora-orchestrate" }
ancora-guard       = { path = "crates/ancora-guard" }
ancora-reason      = { path = "crates/ancora-reason" }
ancora-ageval      = { path = "crates/ancora-ageval" }
```

All types are `Send + Sync` unless documented otherwise. Journals use `Vec` append
semantics and are not thread-safe by default; wrap with `Mutex` for concurrent use.

## Go (sdk/go/*)

The Go SDK examples in `sdk/go/` each demonstrate a single capability slice:

| Directory | Demonstrates |
|---|---|
| `sdk/go/single-agent` | Basic AgentTask dispatch |
| `sdk/go/multi-agent-verifier` | Fan-out and result verification |
| `sdk/go/streaming-chat` | Streaming partial results |
| `sdk/go/structured-output` | JSON schema-constrained output |
| `sdk/go/human-in-loop` | Approval gates |
| `sdk/go/mcp-tool` | MCP tool registration |
| `sdk/go/rag-lancedb` | RAG with LanceDB |
| `sdk/go/sqlite-persistence` | Durable state via SQLite |

Port the canonical parity values from `test_parity.rs` to validate numeric correctness.

## Python

When wrapping ancora crates via PyO3 or via a subprocess bridge:
- Pass u64 ticks as Python `int`
- Serialize journal entries as JSON for interop
- Map `Option<T>` return values to `T | None`

## Numeric Parity

All metric functions must produce results matching the canonical table in
`determinism-notes.md` to within floating-point epsilon. Use exact integer
arithmetic where possible (count numerator / count denominator) to avoid drift.
