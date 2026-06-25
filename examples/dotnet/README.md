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
