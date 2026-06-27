# Verifier and Consensus

The verifier pattern runs a second agent that checks the first agent's output
before it is accepted.

## Pattern

```go
primarySpec  := ancora.NewAgentSpec("llama3", "Answer the question.")
verifierSpec := ancora.NewAgentSpec("llama3",
    "You are a verifier. Check the answer and respond with {\"verdict\":\"pass\"} or {\"verdict\":\"fail\", \"reason\":\"...\"}.")

agent, _ := ancora.NewAgent()
defer agent.Close()

// Run primary
primary, _ := agent.Run(primarySpec)
primaryEvents, _ := primary.CollectAll()
primaryOutput := extractOutput(primaryEvents)

// Run verifier with primary output as context
verifierSpec.Instructions += "\n\nAnswer to verify: " + primaryOutput
verifier, _ := agent.Run(verifierSpec)
verifierEvents, _ := verifier.CollectAll()
verdict := parseVerdict(verifierEvents)

if verdict == "pass" {
    fmt.Println("Accepted:", primaryOutput)
} else {
    fmt.Println("Rejected, retrying...")
}
```

## Consensus with N verifiers

For higher confidence, run N verifiers concurrently and require a majority:

```go
const N = 3
results := make([]string, N)
var wg sync.WaitGroup
for i := range results {
    wg.Add(1)
    go func(i int) {
        defer wg.Done()
        h, _ := agent.Run(verifierSpec)
        results[i] = parseVerdict(h.CollectAll())
    }(i)
}
wg.Wait()
passes := countPasses(results)
```

## See also

- [Multi-agent graph](multi-agent.md)
