using System;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Ancora;

/// <summary>
/// JSON serialization helpers shared across the SDK.
/// Uses snake_case naming and omits null properties.
/// </summary>
internal static class Wire
{
    internal static readonly JsonSerializerOptions Options = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
        Converters = { new RunEventJsonConverter() },
    };

    /// <summary>
    /// Serialize an AgentSpec to UTF-8 JSON bytes for the FFI StartRun call.
    /// </summary>
    internal static byte[] EncodeAgentSpec(AgentSpec spec) =>
        JsonSerializer.SerializeToUtf8Bytes(spec, Options);

    /// <summary>
    /// Serialize a GraphSpec to UTF-8 JSON bytes for the FFI StartRun call.
    /// </summary>
    internal static byte[] EncodeGraphSpec(GraphSpec graph) =>
        JsonSerializer.SerializeToUtf8Bytes(graph, Options);

    /// <summary>
    /// Parse a RunEvent from a JSON string received from FFI PollEvent.
    /// </summary>
    internal static RunEvent ParseEvent(string json)
    {
        return JsonSerializer.Deserialize<RunEvent>(json, Options)
            ?? throw new InvalidOperationException("Deserializing run event returned null");
    }

    /// <summary>
    /// Parse a RunEvent from UTF-8 JSON bytes.
    /// </summary>
    internal static RunEvent ParseEvent(ReadOnlySpan<byte> bytes)
    {
        return JsonSerializer.Deserialize<RunEvent>(bytes, Options)
            ?? throw new InvalidOperationException("Deserializing run event returned null");
    }

    /// <summary>
    /// Return a JSON string as UTF-8 bytes for the FFI.
    /// </summary>
    internal static byte[] EncodeDecision(string decision) =>
        Encoding.UTF8.GetBytes(decision);
}
