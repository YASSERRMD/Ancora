# Streaming

Consume events in real time as the model generates tokens.

## EventChan

```go
handle, _ := agent.Run(spec)
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
defer cancel()

eventCh, errCh := handle.EventChan(ctx)
for {
    select {
    case ev, ok := <-eventCh:
        if !ok { return }
        if tok, ok := ev.(*ancora.TokenEvent); ok {
            fmt.Print(tok.Text)
        }
        if _, ok := ev.(*ancora.CompletedEvent); ok {
            return
        }
    case err := <-errCh:
        fmt.Println("stream error:", err)
        return
    }
}
```

## Counting events

```go
var count int
for ev := range eventCh {
    count++
    _ = ev
}
fmt.Println("received", count, "events")
```

## CollectAll vs streaming

Use `CollectAll()` when you only need the final output and do not need to
react to tokens as they arrive. Use `EventChan` when you need to stream
tokens to a client (e.g. a web SSE endpoint) or react to intermediate
events.

## See also

- [Quickstart](quickstart.md)
- [Human-in-the-loop](human-in-the-loop.md)
