using System.Text.Json;

namespace Ancora;

/// <summary>
/// A managed delegate that handles a tool invocation.
/// Receives the full tool input as a JsonElement and returns
/// the tool result as a JSON string.
/// </summary>
public delegate string ToolHandler(JsonElement input);
