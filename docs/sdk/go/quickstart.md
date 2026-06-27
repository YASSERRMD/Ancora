# Quickstart: Single Agent

Run your first Ancora agent in Go in under 10 lines.

## Prerequisites

- [Install](install.md) complete
- Ollama running locally: `ollama serve`

## Minimal example

```go
package main

import (
    "fmt"
    "ancora.io/sdk"
)

func main() {
    agent, err := ancora.NewAgent()
    if err != nil { panic(err) }
    defer agent.Close()

    spec := ancora.NewAgentSpec("llama3", "You are a helpful assistant.")
    run, err := agent.Run(spec)
    if err != nil { panic(err) }

    events, err := run.CollectAll()
    if err != nil { panic(err) }

    for _, ev := range events {
        if tok, ok := ev.(*ancora.TokenEvent); ok {
            fmt.Print(tok.Text)
        }
    }
    fmt.Println()
}
```

## What just happened?

1. `NewAgent()` creates a runtime and starts the FFI bridge.
2. `NewAgentSpec` wraps a model ID and system prompt.
3. `agent.Run(spec)` starts the agent loop and returns a `RunHandle`.
4. `CollectAll()` drains all events (blocks until `CompletedEvent`).
5. We print each `TokenEvent.Text` to reconstruct the model's reply.

## Next steps

- Add tools: [Defining tools](tools.md)
- Stream events in real time: [Streaming](streaming.md)
- Build a multi-agent graph: [Multi-agent](multi-agent.md)
