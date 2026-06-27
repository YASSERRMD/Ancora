# Deployment (Java)

## Fat JAR

```xml
<!-- pom.xml -->
<plugin>
    <groupId>org.apache.maven.plugins</groupId>
    <artifactId>maven-shade-plugin</artifactId>
    <executions>
        <execution>
            <phase>package</phase>
            <goals><goal>shade</goal></goals>
            <configuration>
                <manifestEntries>
                    <Main-Class>com.example.AgentApp</Main-Class>
                </manifestEntries>
            </configuration>
        </execution>
    </executions>
</plugin>
```

```bash
mvn package
cp target/release/libancora_ffi.so .
LD_LIBRARY_PATH=. java -jar target/myapp-shaded.jar
```

## Docker

```dockerfile
FROM eclipse-temurin:21-jre-alpine

RUN apk add --no-cache libgcc libstdc++

WORKDIR /app
COPY target/myapp-shaded.jar .
COPY libancora_ffi.so .

ENV ANCORA_MODEL_URL=http://ollama:11434
ENV LD_LIBRARY_PATH=/app

ENTRYPOINT ["java", "-jar", "myapp-shaded.jar"]
```

## Spring Boot

```java
@SpringBootApplication
public class AgentApplication {
    public static void main(String[] args) { SpringApplication.run(AgentApplication.class, args); }
}

@Service
public class AgentService {
    private final Runtime rt = new Runtime();
    private final Agent agent = new Agent(rt);
    private final AgentSpec spec = new AgentSpec("llama3", "Answer.", List.of(), 4096, 0.7f);

    @PostMapping("/ask")
    public String ask(@RequestParam String prompt) throws Exception {
        for (var ev : agent.run(spec, prompt).events())
            if (ev instanceof RunEvent.Completed c) return c.output();
        return "";
    }
}
```

## Air-gapped deployment

1. Build the fat JAR on a connected machine.
2. Copy JAR, `libancora_ffi.so`, and model weights to the air-gapped host.
3. Pull Ollama (offline bundle) or use a local inference server.
4. Run with `LD_LIBRARY_PATH=. java -jar myapp-shaded.jar`.

## See also

- [Install](install.md)
- [Deployment models concept](../../concepts/deployment-models.md)
