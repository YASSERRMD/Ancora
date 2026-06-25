package io.ancora;

import com.fasterxml.jackson.annotation.JsonInclude;
import java.util.List;
import java.util.Map;

@JsonInclude(JsonInclude.Include.NON_NULL)
public record ToolInputSchema(
    String type,
    Map<String, ToolInputProperty> properties,
    List<String> required
) {
    public ToolInputSchema {
        if (type == null || type.isBlank()) type = "object";
    }
}
