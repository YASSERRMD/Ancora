using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
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

    private sealed record RuntimeConfigWire(ProviderConfig? Provider, MemoryConfig? Memory);

    /// <summary>
    /// Serialize a ProviderConfig and/or MemoryConfig to UTF-8 JSON bytes for
    /// the FFI RuntimeNewWithConfig call. Either may be null; null fields are
    /// omitted from the JSON entirely (see <see cref="Options"/>), so a
    /// runtime configured with only one of the two never accidentally
    /// resets the other to its default.
    /// </summary>
    internal static byte[] EncodeRuntimeConfig(ProviderConfig? provider, MemoryConfig? memory = null) =>
        JsonSerializer.SerializeToUtf8Bytes(new RuntimeConfigWire(provider, memory), Options);

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

    private sealed record ToolDecisionWire(string ResultJson, bool IsError);

    /// <summary>
    /// Serialize a typed tool-call decision to UTF-8 JSON bytes for the FFI
    /// ancora_run_resume call.
    /// </summary>
    internal static byte[] EncodeToolDecision(string resultJson, bool isError) =>
        JsonSerializer.SerializeToUtf8Bytes(new ToolDecisionWire(resultJson, isError), Options);

    // ---- memory / vector store wire shapes --------------------------------

    private sealed record CollectionSpecWire(string Name, int Dimensions, string Distance);

    /// <summary>
    /// Serialize a collection spec to UTF-8 JSON bytes for the FFI
    /// ancora_memory_create_collection call.
    /// </summary>
    internal static byte[] EncodeCollectionSpec(string name, int dimensions, string distance) =>
        JsonSerializer.SerializeToUtf8Bytes(new CollectionSpecWire(name, dimensions, distance), Options);

    private sealed record PointWire(ulong Id, float[] Vector, IReadOnlyDictionary<string, object?>? Payload);

    /// <summary>
    /// Serialize points to a UTF-8 JSON array for the FFI
    /// ancora_memory_upsert call.
    /// </summary>
    internal static byte[] EncodePoints(IReadOnlyList<VectorPoint> points)
    {
        var wire = points.Select(p => new PointWire(p.Id, p.Vector, p.Payload)).ToArray();
        return JsonSerializer.SerializeToUtf8Bytes(wire, Options);
    }

    private sealed record QueryRequestWire(
        float[] Vector, int TopK, double? ScoreThreshold, JsonNode? Filter);

    /// <summary>
    /// Serialize a similarity query to UTF-8 JSON bytes for the FFI
    /// ancora_memory_query call.
    /// </summary>
    internal static byte[] EncodeQueryRequest(
        float[] vector, int topK, double? scoreThreshold, VectorFilter? filter = null) =>
        JsonSerializer.SerializeToUtf8Bytes(
            new QueryRequestWire(vector, topK, scoreThreshold, filter is null ? null : EncodeFilter(filter)),
            Options);

    private sealed record HybridQueryRequestWire(
        float[] DenseVector, string Keyword, int TopK, float Alpha, double? ScoreThreshold, JsonNode? Filter);

    /// <summary>
    /// Serialize a hybrid (dense-vector + keyword) query to UTF-8 JSON bytes
    /// for the FFI ancora_memory_hybrid_query call.
    /// </summary>
    internal static byte[] EncodeHybridQueryRequest(
        float[] denseVector,
        string keyword,
        int topK,
        float alpha,
        double? scoreThreshold,
        VectorFilter? filter) =>
        JsonSerializer.SerializeToUtf8Bytes(
            new HybridQueryRequestWire(
                denseVector, keyword, topK, alpha, scoreThreshold,
                filter is null ? null : EncodeFilter(filter)),
            Options);

    /// <summary>
    /// Serialize a bare filter expression to UTF-8 JSON bytes for the FFI
    /// ancora_memory_delete_by_filter call.
    /// </summary>
    internal static byte[] EncodeFilterBytes(VectorFilter filter) =>
        Encoding.UTF8.GetBytes(EncodeFilter(filter).ToJsonString());

    /// <summary>
    /// Encode a <see cref="VectorFilter"/> tree into the
    /// <c>{"eq":["field","value"]}</c>-shaped JSON the FFI's filter decoder
    /// expects (see ancora-ffi's memory_backend.rs decode_filter).
    /// </summary>
    private static JsonNode EncodeFilter(VectorFilter filter) => filter switch
    {
        EqFilter f => FilterPair("eq", f.Field, f.Value),
        NeFilter f => FilterPair("ne", f.Field, f.Value),
        GtFilter f => FilterPair("gt", f.Field, f.Value),
        LtFilter f => FilterPair("lt", f.Field, f.Value),
        AndFilter f => new JsonObject { ["and"] = new JsonArray(EncodeFilter(f.Left), EncodeFilter(f.Right)) },
        OrFilter f => new JsonObject { ["or"] = new JsonArray(EncodeFilter(f.Left), EncodeFilter(f.Right)) },
        _ => throw new NotSupportedException($"Unknown VectorFilter subtype: {filter.GetType()}"),
    };

    private static JsonNode FilterPair(string op, string field, object value) =>
        new JsonObject
        {
            [op] = new JsonArray(JsonValue.Create(field), JsonSerializer.SerializeToNode(value)),
        };

    /// <summary>
    /// Serialize point ids to a UTF-8 JSON array for the FFI
    /// ancora_memory_delete call. Uses no naming policy since the wire
    /// format is a bare array of numbers, not an object.
    /// </summary>
    internal static byte[] EncodeIds(IEnumerable<ulong> ids) =>
        JsonSerializer.SerializeToUtf8Bytes(ids.ToArray());

    private sealed record ScoredPointWire(ulong Id, float Score, Dictionary<string, JsonElement>? Payload);

    /// <summary>
    /// Parse the JSON array returned by the FFI ancora_memory_query call
    /// into typed scored points.
    /// </summary>
    internal static IReadOnlyList<ScoredVectorPoint> ParseScoredPoints(ReadOnlySpan<byte> bytes)
    {
        var wire = JsonSerializer.Deserialize<List<ScoredPointWire>>(bytes, Options)
            ?? new List<ScoredPointWire>();
        return wire
            .Select(w => new ScoredVectorPoint(w.Id, w.Score, w.Payload ?? new Dictionary<string, JsonElement>()))
            .ToList();
    }

    private sealed record CollectionInfoWire(string Name, int Dimensions, ulong PointCount, string Distance);

    /// <summary>
    /// Parse the JSON object returned by the FFI
    /// ancora_memory_describe_collection call.
    /// </summary>
    internal static CollectionInfo ParseCollectionInfo(ReadOnlySpan<byte> bytes)
    {
        var wire = JsonSerializer.Deserialize<CollectionInfoWire>(bytes, Options)
            ?? throw new InvalidOperationException("Deserializing collection info returned null");
        return new CollectionInfo(wire.Name, wire.Dimensions, wire.PointCount, wire.Distance);
    }

    /// <summary>
    /// Parse the <c>{"deleted_count":N}</c> JSON object returned by the FFI
    /// ancora_memory_delete_by_filter call.
    /// </summary>
    internal static ulong ParseDeletedCount(ReadOnlySpan<byte> bytes)
    {
        using var doc = JsonDocument.Parse(bytes.ToArray());
        return doc.RootElement.GetProperty("deleted_count").GetUInt64();
    }
}
