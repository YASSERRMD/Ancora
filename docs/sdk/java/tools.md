# Defining Tools (Java)

Tools are `ToolSpec` instances registered in a `List<ToolSpec>` and passed to
an `AgentSpec`. The agent calls them during a run; return values are injected
back into the model context.

## Register a tool

```java
import io.ancora.*;
import java.util.List;
import java.util.Map;

var weatherTool = new ToolSpec(
    "get_weather",
    "Return the current weather for a city.",
    new ToolInputSchema(
        "object",
        Map.of("city", new ToolInputProperty("string", "City name")),
        List.of("city")
    ),
    args -> {
        String city = args.get("city").asText();
        return city + ": 22 C, sunny";
    }
);

var spec = new AgentSpec(
    "llama3",
    "Use get_weather to answer weather questions.",
    List.of(weatherTool),
    4096, 0.7f
);
```

## Multiple tools

```java
var priceTool = new ToolSpec(
    "get_price",
    "Look up the price of a product by SKU.",
    new ToolInputSchema(
        "object",
        Map.of(
            "sku", new ToolInputProperty("string", "Product SKU"),
            "currency", new ToolInputProperty("string", "Currency code (USD, EUR, GBP)")
        ),
        List.of("sku")
    ),
    args -> {
        String sku = args.get("sku").asText();
        String currency = args.has("currency") ? args.get("currency").asText() : "USD";
        return sku + ": 29.99 " + currency;
    }
);

var spec = new AgentSpec(
    "llama3",
    "Answer questions about weather and prices.",
    List.of(weatherTool, priceTool),
    4096, 0.7f
);
```

## Effect classes

```java
import io.ancora.EffectClass;

var writeTool = new ToolSpec(
    "write_file",
    "Write content to a file.",
    /* schema */,
    EffectClass.WRITE,
    args -> { /* write file */ return "ok"; }
);
```

## See also

- [Structured output](structured-output.md)
- [Policy](policy.md)
