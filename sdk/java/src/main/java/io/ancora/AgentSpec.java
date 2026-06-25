package io.ancora;

import com.fasterxml.jackson.annotation.JsonInclude;
import java.util.List;

@JsonInclude(JsonInclude.Include.NON_NULL)
public record AgentSpec(
    String model,
    String instructions,
    List<ToolSpec> tools,
    Integer maxTokens,
    Double temperature
) {}
