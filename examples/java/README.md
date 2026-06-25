# Ancora Java Examples

Runnable examples using the Ancora Java SDK.

## Prerequisites

1. Java 22 or later
2. Gradle 8.10 or later (or use `gradle/actions/setup-gradle@v4` in CI)
3. The `ancora_ffi` native library built from the repo root:

```bash
cargo build -p ancora-ffi --release
```

4. Native library on the search path:

```bash
# macOS
export DYLD_LIBRARY_PATH="$PWD/target/release:$DYLD_LIBRARY_PATH"
# Linux
export LD_LIBRARY_PATH="$PWD/target/release:$LD_LIBRARY_PATH"
```

## Examples

### single-agent

Streams events from a single agent run and prints each to stdout.

```bash
cd examples/java/single-agent
gradle run
```

### mcp-tool-use

Registers two annotation-based tool callbacks (`getWeather`, `celsiusToFahrenheit`)
and runs an agent that can call them. Prints tool_call events and the cost summary.

```bash
cd examples/java/mcp-tool-use
gradle run
```

## How the composite build works

Each example uses `includeBuild` in its `settings.gradle` to pull in
`sdk/java` as a local dependency. No published artifact is required; Gradle
resolves `io.ancora:ancora-java-sdk` directly from the source tree.
