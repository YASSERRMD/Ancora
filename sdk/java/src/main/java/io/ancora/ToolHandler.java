package io.ancora;

import com.fasterxml.jackson.databind.JsonNode;

@FunctionalInterface
public interface ToolHandler {
    String handle(JsonNode input) throws Exception;
}
