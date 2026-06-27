# Policy and Data Residency (Python)

Enforce which providers and regions an agent is allowed to use.

## Configuration

```python
from ancora import AgentSpec, PolicySpec

spec = AgentSpec(
    model="claude-3-5-haiku-20241022",
    instructions="Answer.",
    policy=PolicySpec(
        allow_regions=["us-east-1", "eu-west-1"],
        deny_providers=["openai-gpt4-global"],
        max_write_tools=3,
    ),
)
```

## Capping write-tool calls

`max_write_tools` limits the number of `WRITE`-effect tool calls per run:

```python
spec = AgentSpec(
    model="llama3",
    instructions="Modify files as needed.",
    policy=PolicySpec(max_write_tools=2),
)
```

If the agent tries to call a third write tool, the run fails with a
`PolicyViolationError`.

## Catching policy violations

```python
from ancora import PolicyViolationError

try:
    result = rt.run(spec, "Overwrite all config files.")
except PolicyViolationError as e:
    print("Policy blocked:", e)
```

## Audit trail

Policy checks are recorded as journal activities with
`activity_kind = "policy_check"`. They are visible in the journal and
replayed correctly.

## See also

- [Providers](providers.md)
- [Observability](observability.md)
- [Policy concept](../../concepts/policy-and-data-sovereignty.md)
