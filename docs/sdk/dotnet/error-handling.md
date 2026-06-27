# Error Handling (.NET)

## Exception hierarchy

```
AncorException
├── NativeException           # P/Invoke failure
├── RunFailedException        # run terminated with an error event
├── PolicyViolationException  # policy rule blocked the run
├── TimeoutException          # run exceeded max runtime
└── JournalException          # journal read/write failure
```

## Catching run failures

```csharp
using Ancora;

try
{
    await foreach (var ev in agent.Run(spec, "What is 2+2?").Events())
    {
        if (ev is CompletedEvent c) Console.WriteLine(c.Output);
    }
}
catch (RunFailedException ex)
{
    Console.Error.WriteLine($"Run failed: {ex.Message} (RunId={ex.RunId})");
}
```

## Retry on transient errors

```csharp
async Task<string> RunWithRetry(Agent agent, AgentSpec spec, string prompt, int maxAttempts = 3)
{
    for (int attempt = 0; attempt < maxAttempts; attempt++)
    {
        try
        {
            string output = "";
            await foreach (var ev in agent.Run(spec, prompt).Events())
                if (ev is CompletedEvent c) output = c.Output;
            return output;
        }
        catch (RunFailedException ex) when (ex.IsTransient && attempt < maxAttempts - 1)
        {
            await Task.Delay(TimeSpan.FromSeconds(Math.Pow(2, attempt)));
        }
    }
    throw new InvalidOperationException("unreachable");
}
```

## Cancellation

```csharp
using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(30));

try
{
    await foreach (var ev in agent.Run(spec, prompt).Events(cts.Token))
    {
        if (ev is TokenEvent t) Console.Write(t.Token);
    }
}
catch (OperationCanceledException)
{
    Console.WriteLine("\nRun cancelled.");
}
```

## See also

- [Troubleshooting](troubleshooting.md)
- [Durability](durability.md)
