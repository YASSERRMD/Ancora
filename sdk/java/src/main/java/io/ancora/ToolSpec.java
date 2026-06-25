package io.ancora;

import com.fasterxml.jackson.annotation.JsonInclude;

@JsonInclude(JsonInclude.Include.NON_NULL)
public record ToolSpec(String name, String description, ToolInputSchema inputSchema) {}
