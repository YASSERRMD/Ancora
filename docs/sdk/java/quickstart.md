# Quickstart (Java)

Run a minimal agent locally in under five minutes.

## Prerequisites

- `io.ancora:ancora-sdk` on the classpath
- Ollama running: `ollama serve && ollama pull llama3`

## Minimal example

```java
import io.ancora.*;
import org.junit.jupiter.api.Assumptions;

public class QuickstartExample {
    public static void main(String[] args) throws Exception {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library not available");

        try (var rt = new Runtime(); var agent = new Agent(rt)) {
            var spec = new AgentSpec(
                "llama3",
                "Answer concisely.",
                List.of(),
                4096,
                0.7f
            );

            for (var ev : agent.run(spec, "What is a durable agent?").events()) {
                if (ev instanceof RunEvent.Completed c) {
                    System.out.println(c.output());
                }
            }
        }
    }
}
```

## What happened

1. `AncoraNative.AVAILABLE` checks whether the native library is loaded.
2. `new Runtime()` initialises the Ancora engine via JNI.
3. `new Agent(rt)` creates an agent; both are `AutoCloseable`.
4. `agent.run(spec, prompt).events()` returns an `Iterable<RunEvent>`.
5. `RunEvent.Completed` contains the final model output.

## Skip pattern in JUnit 5

```java
import org.junit.jupiter.api.Assumptions;

@Test
void runsSingleAgent() {
    Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library not available");
    // test body
}
```

## Next steps

- [Defining tools](tools.md)
- [Structured output](structured-output.md)
- [Streaming](streaming.md)
