using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Ancora;

/// <summary>
/// Base type for all events emitted by a run.
/// Use a switch expression on the concrete subtype to handle each event.
/// </summary>
[JsonConverter(typeof(RunEventJsonConverter))]
public abstract record RunEvent(string Kind, string RunId);

/// <summary>
/// Emitted when a run is first created.
/// The Spec field is the serialized agent spec passed to StartRun.
/// </summary>
public sealed record StartedEvent(string RunId, string Spec)
    : RunEvent("started", RunId);

/// <summary>
/// Emitted for each streaming token from the model.
/// </summary>
public sealed record TokenEvent(string RunId, string Text)
    : RunEvent("token", RunId);

/// <summary>
/// Emitted when a run reaches its final completed state.
/// </summary>
public sealed record CompletedEvent(string RunId)
    : RunEvent("completed", RunId);

/// <summary>
/// Emitted after a suspended run receives a human decision and resumes.
/// </summary>
public sealed record ResumedEvent(string RunId, string Decision)
    : RunEvent("resumed", RunId);

/// <summary>
/// Emitted when the model requests a tool invocation.
/// The Input field is the JSON-serialized tool arguments.
/// </summary>
public sealed record ToolCallEvent(string RunId, string Name, string Input)
    : RunEvent("tool_call", RunId);

/// <summary>
/// Custom JSON converter for the RunEvent discriminated union.
/// Dispatches on the "kind" field to the correct subtype.
/// </summary>
internal sealed class RunEventJsonConverter : JsonConverter<RunEvent>
{
    public override RunEvent? Read(
        ref Utf8JsonReader reader,
        Type typeToConvert,
        JsonSerializerOptions options)
    {
        using var doc = JsonDocument.ParseValue(ref reader);
        var root = doc.RootElement;

        var kind = root.GetProperty("kind").GetString() ?? "";
        var runId = root.GetProperty("run_id").GetString() ?? "";

        return kind switch
        {
            "started" => new StartedEvent(
                runId,
                root.TryGetProperty("spec", out var spec) ? spec.GetString() ?? "" : ""),
            "token" => new TokenEvent(
                runId,
                root.TryGetProperty("text", out var text) ? text.GetString() ?? "" : ""),
            "completed" => new CompletedEvent(runId),
            "resumed" => new ResumedEvent(
                runId,
                root.TryGetProperty("decision", out var dec) ? dec.GetString() ?? "" : ""),
            "tool_call" => new ToolCallEvent(
                runId,
                root.TryGetProperty("name", out var name) ? name.GetString() ?? "" : "",
                root.TryGetProperty("input", out var input) ? input.GetString() ?? "" : ""),
            _ => throw new JsonException($"Unknown run event kind: {kind}")
        };
    }

    public override void Write(Utf8JsonWriter writer, RunEvent value, JsonSerializerOptions options)
    {
        throw new NotSupportedException("RunEvent serialization is not supported");
    }
}
