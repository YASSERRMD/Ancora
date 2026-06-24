# multi-agent-verifier

Starts a main agent and a verifier agent concurrently and collects both runs.

## Run

```bash
go run ./examples/multi-agent-verifier
```

## What it shows

- Starting multiple `TransportAgent` runs in parallel goroutines
- Using `sync.WaitGroup` to wait for both runs to complete
- Collecting and printing events from each run independently
