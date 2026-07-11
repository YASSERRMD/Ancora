# Ancora .NET Examples

Runnable examples showing the Ancora .NET SDK.

## Prerequisites

1. .NET 8 SDK
2. Build the native library:
   ```bash
   # from the repo root
   cargo build -p ancora-ffi --release
   ```
3. Set the native library path so examples can find it:
   ```bash
   # Linux
   export LD_LIBRARY_PATH="$PWD/target/release:$LD_LIBRARY_PATH"
   # macOS
   export DYLD_LIBRARY_PATH="$PWD/target/release:$DYLD_LIBRARY_PATH"
   ```

## Run examples

```bash
# from the repo root

# Single agent (streams events from a local model)
dotnet run --project examples/dotnet/single-agent

# Multi-agent verifier pattern (drafter + verifier)
dotnet run --project examples/dotnet/multi-agent

# Retrieval-augmented, structured-output compliance review (runs fully
# offline against an in-process demo server by default -- no setup needed)
dotnet run --project examples/dotnet/compliance-review
```

## Examples

### single-agent

Demonstrates the minimal pattern:
- Build an `AgentSpec` with model, instructions, and max tokens
- Create an `Agent`, call `Run(spec)` to get a `RunHandle`
- Stream events with `async foreach` over `EventsAsync()`
- Print cost summary at the end

### multi-agent

Demonstrates the verifier pattern:
- Two agents sharing a single `Runtime`
- A drafter generates an answer to a question
- A verifier reviews the draft and returns APPROVED or REVISE
- Both agents cost summaries are printed at the end

### compliance-review

The end-to-end, Attestra-shaped example: embeddings, retrieval, tool
calling, and structured output against a real NVIDIA NIM-compatible
endpoint, in one flow.
- `NimEmbedder` embeds two contract clauses and indexes them with
  `Runtime.Upsert`
- The review question is embedded and `Runtime.Query` retrieves the
  relevant clause
- The review agent calls a registered tool (`lookup_precedent`) before
  producing its verdict
- The final output is deserialized into a typed `ComplianceVerdict` record
- Runs fully offline by default against a small in-process demo server; set
  `NIM_BASE_URL` (and `NVIDIA_API_KEY` for hosted NIM) to point it at a real
  deployment instead -- switching is a base-url change only
