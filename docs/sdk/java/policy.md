# Policy and Data Residency (Java)

## Configuration

```java
import io.ancora.*;
import java.util.List;

var spec = new AgentSpec.Builder()
    .model("claude-3-5-haiku-20241022")
    .instructions("Answer.")
    .policy(new PolicySpec(
        List.of("us-east-1", "eu-west-1"),   // allowRegions
        List.of("openai-gpt4-global"),         // denyProviders
        3                                      // maxWriteTools
    ))
    .build();
```

## Capping write-tool calls

```java
var spec = new AgentSpec.Builder()
    .model("llama3")
    .instructions("Modify files as needed.")
    .policy(new PolicySpec(List.of(), List.of(), 2))
    .build();
```

If the agent tries to call a third write tool, the run fails with a
`PolicyViolationException`.

## Catching policy violations

```java
try {
    for (var ev : agent.run(spec, "Overwrite all config files.").events()) {
        if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
    }
} catch (PolicyViolationException e) {
    System.err.println("Policy blocked: " + e.getMessage());
}
```

## Audit trail

Policy checks are journalled as `ActivityRecorded` events with
`activity_kind = "policy_check"`. They appear in the journal and are
replayed correctly on JVM restart.

## See also

- [Providers](providers.md)
- [Observability](observability.md)
- [Policy concept](../../concepts/policy-and-data-sovereignty.md)
