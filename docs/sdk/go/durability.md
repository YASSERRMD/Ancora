# Durability and Restart Recovery

Use `StoringTransport` + `OpenSqliteStore` to make runs survive process
restarts.

## Setup

```go
store, err := ancora.OpenSqliteStore("./data/journal.db")
if err != nil { panic(err) }
defer store.Close()

rt, _ := ancora.NewRuntime()
transport := ancora.NewCgoTransport(rt)
storing   := ancora.NewStoringTransport(transport, store)

agent := ancora.NewTransportAgent(storing)
```

## Run and replay

```go
spec := ancora.NewAgentSpec("llama3", "Persist my events.")
handle, _ := agent.Run(spec)
runID := handle.RunID()

events, _ := handle.CollectAll()
fmt.Println("run ID:", runID, "events:", len(events))

// Simulate restart: create a new agent from the same store
agent2 := ancora.NewTransportAgent(ancora.NewStoringTransport(
    ancora.NewCgoTransport(rt), store,
))
// Agent2 can replay from the journal using the stored run ID
```

## In-memory journal (tests)

```go
type inMemJournal struct { events map[string][]ancora.RunEvent }

// implement ancora.JournalStore
```

## See also

- [Durability concept](../../concepts/durability-and-replay.md)
