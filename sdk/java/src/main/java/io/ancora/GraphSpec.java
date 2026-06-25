package io.ancora;

import java.util.List;

public record GraphSpec(List<GraphNode> nodes, List<GraphEdge> edges) {}
