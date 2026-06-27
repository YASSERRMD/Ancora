# Install (Java)

## Requirements

- Java 17 or later
- Maven 3.8+ or Gradle 8+
- A Rust toolchain to build the native library (or use a pre-built binary)

## Maven

```xml
<dependency>
    <groupId>io.ancora</groupId>
    <artifactId>ancora-sdk</artifactId>
    <version>0.1.0</version>
</dependency>
```

## Gradle

```groovy
implementation 'io.ancora:ancora-sdk:0.1.0'
```

## Build from source

```bash
# 1. Build the native Rust library
cargo build --release -p ancora-ffi

# 2. Set the library path so JNI can find it
export LD_LIBRARY_PATH="$(pwd)/target/release:$LD_LIBRARY_PATH"

# 3. Build and install the SDK locally
cd sdk/java && mvn install
```

## Runtime library path

The native library must be on the JVM library path:

```bash
# Linux / macOS
export LD_LIBRARY_PATH=/path/to/native:$LD_LIBRARY_PATH

# Windows
set PATH=C:\path\to\native;%PATH%
```

Or pass it as a JVM flag:

```bash
java -Djava.library.path=/path/to/native -jar myapp.jar
```

## Check availability at runtime

```java
import io.ancora.AncoraNative;

if (AncoraNative.AVAILABLE) {
    System.out.println("Native library loaded.");
} else {
    System.out.println("Native library not found -- tests will be skipped.");
}
```

## Runtime prerequisites

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"   # Ollama (default)
ollama pull llama3
```

## See also

- [Quickstart](quickstart.md)
- [Providers](providers.md)
