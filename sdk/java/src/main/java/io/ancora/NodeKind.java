package io.ancora;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum NodeKind {
    AGENT, FUNCTION, SUBGRAPH;

    @JsonValue
    public String toJson() {
        return name().toLowerCase();
    }

    @JsonCreator
    public static NodeKind fromJson(String value) {
        if (value == null) return AGENT;
        return switch (value.toLowerCase()) {
            case "function" -> FUNCTION;
            case "subgraph" -> SUBGRAPH;
            default -> AGENT;
        };
    }
}
