# GraalVM Native Image (Java)

Build the Ancora Java SDK into a GraalVM native image for faster startup and
lower memory usage.

## Prerequisites

- GraalVM CE or EE 21+
- `native-image` tool installed: `gu install native-image`
- The native Ancora library compiled for the same platform

## Maven configuration

```xml
<plugin>
    <groupId>org.graalvm.buildtools</groupId>
    <artifactId>native-maven-plugin</artifactId>
    <version>0.10.2</version>
    <configuration>
        <imageName>ancora-agent</imageName>
        <mainClass>com.example.AgentApp</mainClass>
        <buildArgs>
            <buildArg>-H:+ReportExceptionStackTraces</buildArg>
            <buildArg>-Djava.library.path=${env.ANCORA_LIB_DIR}</buildArg>
        </buildArgs>
    </configuration>
    <executions>
        <execution>
            <id>build-native</id>
            <goals><goal>compile-no-fork</goal></goals>
            <phase>package</phase>
        </execution>
    </executions>
</plugin>
```

## Build

```bash
export ANCORA_LIB_DIR=/path/to/target/release
mvn -Pnative package
```

This produces `target/ancora-agent` -- a self-contained binary.

## JNI reflection configuration

GraalVM native image requires explicit registration of JNI classes.
Ancora provides a `reflection-config.json` in the SDK JAR under
`META-INF/native-image/`. It is picked up automatically when using the
native-maven-plugin.

## Runtime

```bash
LD_LIBRARY_PATH=/path/to/target/release ./target/ancora-agent
```

Startup time is typically under 50 ms. Heap usage is 10-30x lower than
the JVM equivalent for small agent workloads.

## See also

- [Deployment](deployment.md)
- [Install](install.md)
