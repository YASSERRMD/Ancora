package io.ancora;

import com.fasterxml.jackson.annotation.JsonInclude;

@JsonInclude(JsonInclude.Include.NON_NULL)
public record GraphNode(String id, NodeKind kind, AgentSpec agentSpec) {}
