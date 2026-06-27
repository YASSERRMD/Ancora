# Policy and Data Residency (.NET)

## Configuration

```csharp
using Ancora;

var spec = new AgentSpec
{
    Model = "claude-3-5-haiku-20241022",
    Instructions = "Answer.",
    Policy = new PolicySpec
    {
        AllowRegions = new List<string> { "us-east-1", "eu-west-1" },
        DenyProviders = new List<string> { "openai-gpt4-global" },
        MaxWriteTools = 3,
    },
};
```

## Capping write-tool calls

```csharp
var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Modify files as needed.",
    Policy = new PolicySpec { MaxWriteTools = 2 },
};
```

If the agent tries to call a third write tool, the run fails with a
`PolicyViolationException`.

## Catching policy violations

```csharp
try
{
    await foreach (var ev in agent.Run(spec, "Overwrite all config files.").Events())
    {
        if (ev is CompletedEvent c) Console.WriteLine(c.Output);
    }
}
catch (PolicyViolationException ex)
{
    Console.Error.WriteLine($"Policy blocked: {ex.Message}");
}
```

## Audit trail

Policy checks are journalled as `ActivityRecorded` events with
`activity_kind = "policy_check"`. They appear in the journal and are replayed
correctly on restart.

## See also

- [Providers](providers.md)
- [Observability](observability.md)
- [Policy concept](../../concepts/policy-and-data-sovereignty.md)
