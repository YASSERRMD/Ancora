# Go SDK API Reference

## Core types

### `Runtime`

```go
rt := ancora.NewRuntime()
defer rt.Close()
```

`NewRuntime` initialises the native Ancora engine via CGo. One `Runtime`
per process is sufficient.

### `Agent`

```go
agent := ancora.NewAgent(rt)
defer agent.Close()
```

Created from a `Runtime`. An `Agent` is safe for concurrent use.

### `AgentSpec`

```go
spec := ancora.NewAgentSpec(model string, instructions string)
spec.Tools    = []ancora.ToolSpec{ ... }
spec.MaxTokens = 4096
spec.Temperature = 0.3
```

### `TransportAgent`

```go
ta := ancora.NewTransportAgent(rt, transport)
```

Wraps an agent with a custom `Transport` (e.g. `StoringTransport` for
durable replay).

### `CgoTransport`

```go
t := ancora.NewCgoTransport(rt)
```

Default transport backed by the native Ancora engine.

### `StoringTransport`

```go
store, _ := ancora.OpenSqliteStore("/var/lib/journal.db")
t := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
```

Wraps any `Transport` and records every activity to a `Store`.

### `OpenSqliteStore`

```go
store, err := ancora.OpenSqliteStore(path string) (*ancora.SqliteStore, error)
```

Opens (or creates) a SQLite journal at the given path.

## Tool types

### `GoToolRegistry`

```go
registry := ancora.NewGoToolRegistry()
registry.Register(name string, spec ancora.ToolSpec, fn interface{}) error
registry.Unregister(name string)
```

### `RuntimeToolkit`

```go
toolkit := ancora.NewRuntimeToolkit(registry)
spec.Toolkit = toolkit
```

Passes a registry into an `AgentSpec`.

### `SchemaFromStruct`

```go
schema := ancora.SchemaFromStruct(v interface{}) map[string]interface{}
```

Generates a JSON Schema from a Go struct using reflection.

## Run types

### `RunHandle`

```go
handle := agent.Run(spec)
events, err := handle.CollectAll()
err = handle.Resume(payload string)
err = handle.ResumeBytes(payload []byte)
```

### `EventChan`

```go
ch := ancora.NewEventChan(ctx, handle)
for ev := range ch.C {
    // process event
}
ch.Wait()
```

## Graph types

### `GraphSpec`

```go
graph := ancora.GraphSpec{
    Nodes: []ancora.GraphNode{ ... },
    Edges: []ancora.GraphEdge{ ... },
}
```

### `GraphNode`

```go
node := ancora.GraphNode{ID: "writer", Spec: writerSpec}
```

### `GraphEdge`

```go
edge := ancora.GraphEdge{From: "writer", To: "verifier"}
```

## Policy types

### `PolicySpec`

```go
policy := &ancora.PolicySpec{
    AllowRegions:  []string{"us-east-1"},
    DenyProviders: []string{"openai-global"},
    MaxWriteTools: 3,
}
spec.Policy = policy
```
