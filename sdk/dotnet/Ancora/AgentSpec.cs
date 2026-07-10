using System.Collections.Generic;

namespace Ancora;

/// <summary>
/// A single property in a tool's input schema.
/// </summary>
/// <param name="Type">JSON Schema type string, e.g. "string", "integer".</param>
/// <param name="Description">Human-readable description shown to the model.</param>
public record ToolInputProperty(string Type, string? Description = null);

/// <summary>
/// JSON Schema object describing a tool's accepted inputs.
/// </summary>
/// <param name="Type">Always "object" per the wire spec.</param>
/// <param name="Properties">Named properties the tool accepts.</param>
/// <param name="Required">Names of required properties.</param>
public record ToolInputSchema(
    string Type = "object",
    Dictionary<string, ToolInputProperty>? Properties = null,
    List<string>? Required = null
);

/// <summary>
/// Describes a single callable tool the model may invoke.
/// </summary>
/// <param name="Name">Machine-readable identifier, unique within the agent.</param>
/// <param name="Description">Prose description sent to the model.</param>
/// <param name="InputSchema">JSON Schema for the tool's input.</param>
public record ToolSpec(
    string Name,
    string Description,
    ToolInputSchema? InputSchema = null
);

/// <summary>
/// Describes a single agent: the model it uses, its system prompt, and its tools.
/// </summary>
/// <param name="Model">Model identifier, e.g. "llama3" or "gpt-4o".</param>
/// <param name="Instructions">System prompt sent before the conversation. Also
/// doubles as the run's initial input: the native run engine has no separate
/// input parameter yet, so this is the whole prompt.</param>
/// <param name="Tools">Tools available to this agent.</param>
/// <param name="MaxTokens">Optional token budget for the model response.</param>
/// <param name="Temperature">Optional sampling temperature (0.0 to 2.0).</param>
/// <param name="MaxSteps">Maximum agent loop iterations before the run fails
/// with a max-steps error. 0 (the default) means unlimited.</param>
public record AgentSpec(
    string Model,
    string Instructions = "",
    List<ToolSpec>? Tools = null,
    int? MaxTokens = null,
    double? Temperature = null,
    int MaxSteps = 0
);

/// <summary>
/// Supported graph node kinds.
/// </summary>
public enum NodeKind
{
    Agent,
    Function,
    Subgraph,
}

/// <summary>
/// A single node in an orchestration graph.
/// </summary>
/// <param name="Id">Unique node identifier within the graph.</param>
/// <param name="Kind">What type of node this is.</param>
/// <param name="AgentSpec">Agent configuration (required when Kind is Agent).</param>
public record GraphNode(string Id, NodeKind Kind, AgentSpec? AgentSpec = null);

/// <summary>
/// A directed edge between two graph nodes.
/// </summary>
/// <param name="From">Source node ID.</param>
/// <param name="To">Target node ID.</param>
/// <param name="Condition">Optional condition expression; null means unconditional.</param>
public record GraphEdge(string From, string To, string? Condition = null);

/// <summary>
/// An explicit orchestration graph composing multiple nodes.
/// </summary>
/// <param name="Nodes">All nodes in the graph.</param>
/// <param name="Edges">Directed edges connecting the nodes.</param>
public record GraphSpec(List<GraphNode> Nodes, List<GraphEdge> Edges);
