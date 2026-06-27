# Multi-Agent Verifier Example

Demonstrates running a primary agent and a verifier agent concurrently using
`Task.WhenAll`, sharing a single `Runtime` instance.

## What it tests

- Two `Agent.Run` calls complete without error
- The two `RunId` values are distinct
- Both task results are non-null after `Task.WhenAll`

## Pattern

```csharp
using var agent = new Agent();

var primarySpec  = new AgentSpec("local-model", "Produce an answer.");
var verifierSpec = new AgentSpec("local-model", "Verify the answer.");

var primaryHandle  = agent.Run(primarySpec);
var verifierHandle = agent.Run(verifierSpec);

var results = await Task.WhenAll(
    primaryHandle.CollectAsync(),
    verifierHandle.CollectAsync()
);

Assert.NotEqual(primaryHandle.RunId, verifierHandle.RunId);
```

## Offline behaviour

`DllNotFoundException` is caught; both tasks resolve before the catch so
the test exits cleanly.
