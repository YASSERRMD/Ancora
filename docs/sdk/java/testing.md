# Testing Your Agents (Java)

All Ancora Java tests run offline by default with JUnit 5.

## Skip when native library is absent

```java
import io.ancora.*;
import org.junit.jupiter.api.*;

public class SingleAgentTest {
    @Test
    void runsSingleAgent() throws Exception {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library not available");

        try (var rt = new Runtime(); var agent = new Agent(rt)) {
            var spec = new AgentSpec("llama3", "Answer.", List.of(), 4096, 0.7f);
            String output = "";

            for (var ev : agent.run(spec, "What is 2+2?").events())
                if (ev instanceof RunEvent.Completed c) output = c.output();

            assertFalse(output.isEmpty());
        }
    }
}
```

## Catch `UnsatisfiedLinkError` as a fallback

```java
@Test
void runsSingleAgent() throws Exception {
    try {
        var rt = new Runtime();
        // test body
    } catch (UnsatisfiedLinkError ignored) {
        // native library not available; skip silently
    }
}
```

## Testing tool logic

Tool functions are plain lambdas. Test them in isolation:

```java
@Test
void weatherToolReturnsCorrectFormat() {
    var result = getWeather("Cairo");
    assertTrue(result.contains("Cairo"));
    assertTrue(result.contains("22 C"));
}
```

## Testing schema construction

```java
@Test
void toolInputSchemaBuildsCorrectly() {
    var schema = new ToolInputSchema(
        "object",
        Map.of("city", new ToolInputProperty("string", "City name")),
        List.of("city")
    );
    assertEquals("object", schema.type());
    assertTrue(schema.properties().containsKey("city"));
}
```

## Testing with in-memory journal

```java
@Test
void journalsARun() throws Exception {
    Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library not available");

    var store = new MemoryStore();
    var rt = new Runtime(new RuntimeOptions().withTransport(new StoringTransport(store)));
    try (var agent = new Agent(rt)) {
        var spec = new AgentSpec("llama3", "Answer.", List.of(), 256, 0.0f);
        for (var ev : agent.run(spec, "ping", new RunOptions().withRunId("test-run-1")).events()) { }
        assertTrue(store.hasRun("test-run-1"));
    }
}
```

## Running tests

```bash
mvn test
# or
./gradlew test
```

## See also

- [Quickstart](quickstart.md)
- [Durability](durability.md)
