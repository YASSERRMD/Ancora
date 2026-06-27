# Policy and Data Residency (Go)

Enforce data residency rules to prevent the agent from calling providers
outside an allowed region.

## Configuration

```go
spec := ancora.NewAgentSpec("claude-3-5-haiku-20241022", "Answer.")
spec.Policy = &ancora.PolicySpec{
    AllowRegions:   []string{"us-east-1", "eu-west-1"},
    DenyProviders:  []string{"openai-gpt4-global"},
    MaxWriteTools:  3,
}
```

## Effect limits

Use `MaxWriteTools` to cap the number of `WRITE` tool calls per run:

```go
spec.Policy = &ancora.PolicySpec{MaxWriteTools: 2}
```

If the agent tries to call a third write tool, the run fails with a policy
violation error.

## Audit logging

Policy evaluations are written to the journal as `ActivityRecorded` events
with `activity_kind = "policy_check"`. They are replayed correctly.

## See also

- [Policy concept](../../concepts/policy-and-data-sovereignty.md)
- [Providers](providers.md)
