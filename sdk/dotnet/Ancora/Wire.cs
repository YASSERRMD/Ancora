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
    /// Case-insensitive options for deserializing a run's structured output,
    /// which is produced by the model according to whatever naming
    /// convention its schema/prompt used -- not necessarily snake_case, so
    /// this deliberately does not reuse <see cref="Options"/>.
    /// </summary>
    internal static readonly JsonSerializerOptions StructuredOutputOptions = new()
    {
        PropertyNameCaseInsensitive = true,
    };

    /// <summary>
    /// Serialize an AgentSpec to UTF-8 JSON bytes for the FFI StartRun call.
    /// </summary>
    internal static byte[] EncodeAgentSpec(AgentSpec spec) =>
        JsonSerializer.SerializeToUtf8Bytes(spec, Options);

    private sealed record RuntimeConfigWire(ProviderConfig Provider);

    /// <summary>
    /// Serialize a ProviderConfig to UTF-8 JSON bytes for the FFI
    /// RuntimeNewWithConfig call.
    /// </summary>
    internal static byte[] EncodeRuntimeConfig(ProviderConfig provider) =>
        JsonSerializer.SerializeToUtf8Bytes(new RuntimeConfigWire(provider), Options);

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
