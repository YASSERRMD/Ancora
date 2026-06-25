# Ancora Java SDK

Local-first agentic framework bindings for Java 22+.
Uses the Foreign Function and Memory API (stable since Java 22) -- no JNI required.

## Prerequisites

- Java 22 or later
- The `ancora_ffi` native library (`.dylib` on macOS, `.so` on Linux, `.dll` on Windows)

### Building the native library

```bash
# from the repo root
cargo build -p ancora-ffi --release
# macOS: target/release/libancora_ffi.dylib
# Linux: target/release/libancora_ffi.so
```

Set the library path before running:

```bash
# Linux
export LD_LIBRARY_PATH="$PWD/target/release:$LD_LIBRARY_PATH"
# macOS
export DYLD_LIBRARY_PATH="$PWD/target/release:$DYLD_LIBRARY_PATH"
```

## Installation

Add to your `build.gradle`:

```groovy
dependencies {
    implementation 'io.ancora:ancora-java-sdk:0.1.0'
}
```

Or with Maven, add to `pom.xml`:

```xml
<dependency>
    <groupId>io.ancora</groupId>
    <artifactId>ancora-java-sdk</artifactId>
    <version>0.1.0</version>
</dependency>
```

## Quickstart: run a single agent

```java
import io.ancora.*;

public class Main {
    public static void main(String[] args) throws Throwable {
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec(
                "llama3",
                "You are a concise assistant. Answer in one sentence.",
                null, null, null);

            RunHandle handle = agent.run(spec);

            for (RunEvent ev : handle.events()) {
                switch (ev) {
                    case RunEvent.Started s ->
                        System.out.println("Run started: " + s.runId());
                    case RunEvent.Token t ->
                        System.out.print(t.text());
                    case RunEvent.Completed c ->
                        System.out.println("\nDone.");
                    default -> {}
                }
            }
        }
    }
}
```

## Quickstart: register a tool

```java
import io.ancora.*;
import java.util.List;

public class WeatherMain {
    public static void main(String[] args) throws Throwable {
        try (Runtime runtime = new Runtime()) {
            List<ToolRegistration> tools = ToolRegistry.registerAll(runtime, new Tools());
            try {
                List<ToolSpec> toolSpecs = tools.stream()
                    .map(ToolRegistration::spec).toList();
                AgentSpec spec = new AgentSpec(
                    "llama3", "Use the get_weather tool.",
                    toolSpecs, null, null);
                new Agent(runtime).run(spec).collectAll().forEach(System.out::println);
            } finally {
                for (ToolRegistration r : tools) r.close();
            }
        }
    }

    static class Tools {
        @Tool(description = "Get the current weather for a city")
        public String getWeather(@ToolInput(description = "the city name") String city) {
            return "{\"city\": \"" + city + "\", \"weather\": \"sunny\"}";
        }
    }
}
```

## Running tests

```bash
# from sdk/java/
gradle test
```

Tests that require the native library are skipped gracefully when it is absent.
Set `ANCORA_NATIVE_LIB_PATH` to the directory containing `libancora_ffi` to enable
integration tests locally.
