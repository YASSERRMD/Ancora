# Migration from LangChain4j and Semantic Kernel Java (Java)

## From LangChain4j

| LangChain4j concept | Ancora equivalent |
|---------------------|-------------------|
| `ChatLanguageModel` | `AgentSpec.model` + `ANCORA_MODEL_URL` |
| `AiServices` | `Agent.run(spec, prompt)` |
| `@Tool` annotation | `ToolSpec` lambda |
| `MessageWindowChatMemory` | `StoringTransport` + `SqliteStore` |
| `StreamingChatLanguageModel` | `RunEvent.Token` in `agent.run(..).events()` |
| `EmbeddingModel` + `EmbeddingStore` | Any vector store + `retrieve` tool |

### Before (LangChain4j)

```java
import dev.langchain4j.model.ollama.OllamaChatModel;
import dev.langchain4j.service.AiServices;

var model = OllamaChatModel.builder().baseUrl("http://localhost:11434").modelName("llama3").build();
var assistant = AiServices.builder(Assistant.class).chatLanguageModel(model).build();
String answer = assistant.answer("What is a durable agent?");
```

### After (Ancora)

```java
import io.ancora.*;

try (var rt = new Runtime(); var agent = new Agent(rt)) {
    var spec = new AgentSpec("llama3", "Answer.", List.of(), 4096, 0.7f);
    for (var ev : agent.run(spec, "What is a durable agent?").events())
        if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
}
```

## From Semantic Kernel Java

| Semantic Kernel concept | Ancora equivalent |
|------------------------|-------------------|
| `Kernel` | `Runtime` |
| `KernelFunction` | `ToolSpec` |
| `KernelPlugin` | `List<ToolSpec>` in `AgentSpec` |
| `IChatCompletionService` | `ANCORA_MODEL_URL` env var |
| `KernelFunctionFromMethod` | `new ToolSpec(name, desc, schema, args -> ...)` |

### Before (Semantic Kernel Java)

```java
import com.microsoft.semantickernel.*;

var kernel = Kernel.builder()
    .withAIService(ChatCompletionService.class,
        OpenAIChatCompletion.builder().withModelId("gpt-4o-mini").build())
    .build();

var result = kernel.invokeAsync("prompt", "What is a durable agent?");
System.out.println(result.getResult());
```

### After (Ancora)

```java
var spec = new AgentSpec("gpt-4o-mini", "Answer.", List.of(), 4096, 0.7f);
for (var ev : agent.run(spec, "What is a durable agent?").events())
    if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
```

## Key differences

- Ancora uses `Iterable<RunEvent>` (or `AutoCloseable` handles) for streaming.
- Durability is built-in via `StoringTransport` (no separate plugin).
- Policy enforcement happens at the native engine level.
- `AncoraNative.AVAILABLE` allows graceful degradation in offline test environments.

## See also

- [Multi-agent graphs](multi-agent.md)
- [Durability](durability.md)
