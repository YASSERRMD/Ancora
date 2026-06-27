# Human-in-the-Loop (Java)

Suspend a run at a tool call boundary and resume it with human input.

## Pattern

```java
import io.ancora.*;
import java.util.List;
import java.util.Scanner;

var approvalTool = new ToolSpec(
    "request_approval",
    "Ask a human to approve an action.",
    new ToolInputSchema("object",
        Map.of("action", new ToolInputProperty("string", "Action to approve")),
        List.of("action")),
    args -> {
        throw new SuspendSignal("Approve this action? " + args.get("action").asText());
    }
);

var spec = new AgentSpec(
    "llama3",
    "Before modifying any file, call request_approval.",
    List.of(approvalTool),
    1024, 0.7f
);

try (var rt = new Runtime(); var agent = new Agent(rt)) {
    var handle = agent.start(spec, "Delete the temp directory.");
    handle.runUntilPause();

    if (handle.status() == RunStatus.PAUSED) {
        System.out.println("Approval required: " + handle.pauseReason());
        System.out.print("Type YES to approve: ");
        String answer = new Scanner(System.in).nextLine();
        handle.resume(answer);
    }

    for (var ev : handle.events()) {
        if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
    }
}
```

## Resume with binary payload

```java
byte[] payload = objectMapper.writeValueAsBytes(Map.of("approved", true));
handle.resumeBytes(payload);
```

## Timeout

```java
var timer = new java.util.Timer();
timer.schedule(new java.util.TimerTask() {
    public void run() { handle.resume("TIMEOUT"); }
}, 30_000L);
```

## See also

- [Streaming](streaming.md)
- [Durability](durability.md)
