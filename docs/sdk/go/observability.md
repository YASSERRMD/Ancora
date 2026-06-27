# Observability and Cost (Go)

Track cost and emit OTEL spans in Go.

## Token cost

```go
handle, _ := agent.Run(spec)
events, _ := handle.CollectAll()

var inputTokens, outputTokens int
for _, ev := range events {
    if tok, ok := ev.(*ancora.TokenEvent); ok {
        outputTokens += estimateTokens(tok.Text)
    }
    if started, ok := ev.(*ancora.StartedEvent); ok {
        inputTokens += estimateTokens(started.Spec)
    }
}
fmt.Printf("estimated tokens: in=%d out=%d\n", inputTokens, outputTokens)
```

## In-process span

```go
type span struct {
    name       string
    start      time.Time
    attrs      map[string]any
    durationMs int64
}

func startSpan(name string) *span {
    return &span{name: name, start: time.Now(), attrs: map[string]any{}}
}

func (s *span) set(key string, val any) { s.attrs[key] = val }

func (s *span) end() int64 {
    s.durationMs = time.Since(s.start).Milliseconds()
    return s.durationMs
}
```

## OTEL export

Set `ANCORA_OTEL_ENDPOINT` to an OTLP HTTP endpoint:

```bash
export ANCORA_OTEL_ENDPOINT=http://localhost:4317
```

Ancora automatically attaches `ancora.run_id`, `ancora.model_id`, and token
counts to every span.

## See also

- [Observability concept](../../concepts/observability-and-otel.md)
