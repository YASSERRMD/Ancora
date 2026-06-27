# Your First Agent

This guide walks through building a minimal agent in each supported SDK and
running it locally without any cloud credentials.

## Prerequisites

- [Ollama](https://ollama.com) running locally: `ollama serve`
- A model pulled: `ollama pull llama3`
- One of: Go 1.21+, Python 3.10+, Node 20+, .NET 8, Java 17, Rust 1.75

## What the agent does

The agent receives a user message ("Summarise this text."), calls a `get_text`
tool to retrieve the text, summarises it, and returns a structured JSON result.

---

## Go

```go
package main

import (
    "encoding/json"
    "fmt"
    ancora "ancora.io/sdk"
)

type Summary struct {
    Headline string `json:"headline"`
    Body     string `json:"body"`
}

func main() {
    rt, _ := ancora.NewRuntime()
    defer rt.Close()

    agent := ancora.NewAgent(rt)
    defer agent.Close()

    registry := ancora.NewGoToolRegistry()
    registry.Register("get_text", ancora.ToolSpec{
        Name:        "get_text",
        Description: "Returns the source text for summarisation.",
        InputSchema: ancora.SchemaFromStruct(struct{}{}),
    }, func() string {
        return "Ancora is a durable agent runtime for Go, Python, Rust, and more."
    })

    spec := ancora.NewAgentSpec("llama3", "Summarise the text from get_text. Return JSON with headline and body.")
    spec.Toolkit = ancora.NewRuntimeToolkit(registry)
    spec.OutputSchema = ancora.SchemaFromStruct(Summary{})

    events, _ := agent.Run(spec).CollectAll()
    last := events[len(events)-1]
    if completed, ok := last.(*ancora.CompletedEvent); ok {
        var summary Summary
        json.Unmarshal([]byte(completed.Output), &summary)
        fmt.Println(summary.Headline)
    }
}
```

---

## Python

```python
import json
from ancora import Runtime, AgentSpec, ToolSpec

rt = Runtime()

def get_text() -> str:
    return "Ancora is a durable agent runtime for Go, Python, Rust, and more."

spec = AgentSpec(
    model="llama3",
    instructions="Summarise the text from get_text. Return JSON with headline and body.",
    tools=[ToolSpec.from_callable("get_text", get_text)],
)

result = rt.run(spec)
summary = json.loads(result.output)
print(summary["headline"])
```

---

## TypeScript

```typescript
import { buildSpec, Runtime } from 'ancora'

const rt = new Runtime()

const spec = buildSpec({
  model: 'llama3',
  instructions: 'Summarise the text from get_text. Return JSON with headline and body.',
  tools: [{
    name: 'get_text',
    description: 'Returns the source text for summarisation.',
    fn: () => 'Ancora is a durable agent runtime for Go, Python, Rust, and more.',
  }],
})

const result = await rt.run(spec)
const summary = JSON.parse(result.output)
console.log(summary.headline)
```

---

## .NET

```csharp
using Ancora;
using System.Text.Json;

var rt = new Runtime();
var agent = new Agent(rt);

var spec = new AgentSpec {
    Model = "llama3",
    Instructions = "Summarise the text from get_text. Return JSON with headline and body.",
    Tools = new List<ToolSpec> {
        new ToolSpec {
            Name = "get_text",
            Description = "Returns the source text for summarisation.",
            Fn = _ => "Ancora is a durable agent runtime for Go, Python, Rust, and more.",
        }
    }
};

var handle = agent.Run(spec);
await foreach (var ev in handle.Events()) {
    if (ev is CompletedEvent completed) {
        var summary = JsonSerializer.Deserialize<Summary>(completed.Output);
        Console.WriteLine(summary?.Headline);
    }
}
```

---

## Java

```java
import io.ancora.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import java.util.Map;

var rt = new Runtime();
var agent = new Agent(rt);

var spec = new AgentSpec(
    "llama3",
    "Summarise the text from get_text. Return JSON with headline and body.",
    List.of(new ToolSpec("get_text", "Returns the source text.", new ToolInputSchema(
        "object", Map.of(), List.of()
    ), args -> "Ancora is a durable agent runtime for Go, Python, Rust, and more.")),
    4096, 0.3f
);

var handle = agent.run(spec);
var mapper = new ObjectMapper();
for (var ev : handle.events()) {
    if (ev instanceof RunEvent.Completed c) {
        var summary = mapper.readTree(c.output());
        System.out.println(summary.get("headline").asText());
    }
}
```

---

## Next steps

- [Multi-agent quickstart](multi-agent.md) -- fan-out orchestration
- [Durability guide](../guides/durability.md) -- crash recovery
- [Streaming guide](../sdk/go/streaming.md) -- token-by-token output
