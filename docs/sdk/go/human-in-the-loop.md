# Human-in-the-Loop

Pause a run and resume it with a human decision.

## Pause

The agent loop emits a `HumanDecisionRequested` event when it needs input.
Your code detects this event and waits:

```go
handle, _ := agent.Run(spec)
events, _ := handle.CollectAll()

for _, ev := range events {
    if _, ok := ev.(*ancora.HumanDecisionRequestedEvent); ok {
        fmt.Println("Waiting for human input...")
        break
    }
}
```

## Resume with a string decision

```go
handle.Resume("approved")
postEvents, _ := handle.CollectAll()
```

## Resume with structured JSON

```go
decision := `{"action":"approve","reason":"reviewed by alice"}`
handle.Resume(decision)
```

## Resume with raw bytes

```go
handle.ResumeBytes([]byte("approved"))
```

## Timeout

Set `HumanDecisionTimeout` in the spec to fail the run automatically if no
decision arrives within the deadline:

```go
spec.HumanDecisionTimeout = 5 * time.Minute
```

## See also

- [Human-in-the-loop concept](../../concepts/human-in-the-loop.md)
