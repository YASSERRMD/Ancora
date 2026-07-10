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
/// The Output field is the run's final text output; use
/// <see cref="StructuredOutputExtensions.Deserialize{T}"/> to parse it as a
/// typed structured-output value.
/// </summary>
public sealed record CompletedEvent(string RunId, string Output = "")
    : RunEvent("completed", RunId);

/// <summary>
/// Emitted when a run fails, e.g. an unreachable or erroring model
/// endpoint. The Error field is a human-readable description.
/// </summary>
public sealed record FailedEvent(string RunId, string Error)
    : RunEvent("failed", RunId);

/// <summary>
/// Emitted after a suspended run receives a human decision and resumes.
/// </summary>
public sealed record ResumedEvent(string RunId, string Decision)
    : RunEvent("resumed", RunId);

/// <summary>
/// Emitted when the run pauses at a tool call registered with
/// <see cref="Runtime.RegisterCallbackRequiringApproval"/> -- the run stays
/// paused until <see cref="RunHandle.Resume"/>/<see cref="Runtime.ResumeRun"/>
/// supplies a decision for <see cref="ToolCallId"/>.
/// </summary>
public sealed record SuspendedEvent(
    string RunId,
    string ToolCallId,
    string ToolName,
    string ArgumentsJson,
    string Prompt)
    : RunEvent("suspended", RunId);

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
            "completed" => new CompletedEvent(
                runId,
                root.TryGetProperty("output", out var output) ? output.GetString() ?? "" : ""),
            "failed" => new FailedEvent(
                runId,
                root.TryGetProperty("error", out var error) ? error.GetString() ?? "" : ""),
            "resumed" => new ResumedEvent(
                runId,
                root.TryGetProperty("decision", out var dec) ? dec.GetString() ?? "" : ""),
            "tool_call" => new ToolCallEvent(
                runId,
                root.TryGetProperty("name", out var name) ? name.GetString() ?? "" : "",
                root.TryGetProperty("input", out var input) ? input.GetString() ?? "" : ""),
            "suspended" => new SuspendedEvent(
                runId,
                root.TryGetProperty("tool_call_id", out var tcId) ? tcId.GetString() ?? "" : "",
                root.TryGetProperty("tool_name", out var tName) ? tName.GetString() ?? "" : "",
                root.TryGetProperty("arguments_json", out var args) ? args.GetString() ?? "" : "",
                root.TryGetProperty("prompt", out var prompt) ? prompt.GetString() ?? "" : ""),
            _ => throw new JsonException($"Unknown run event kind: {kind}")
        };
    }

    public override void Write(Utf8JsonWriter writer, RunEvent value, JsonSerializerOptions options)
    {
        writer.WriteStartObject();
        writer.WriteString("kind", value.Kind);
        writer.WriteString("run_id", value.RunId);
        switch (value)
        {
            case StartedEvent started:
                writer.WriteString("spec", started.Spec);
                break;
            case TokenEvent token:
                writer.WriteString("text", token.Text);
                break;
            case CompletedEvent completed:
                writer.WriteString("output", completed.Output);
                break;
            case FailedEvent failed:
                writer.WriteString("error", failed.Error);
                break;
            case ResumedEvent resumed:
                writer.WriteString("decision", resumed.Decision);
                break;
            case ToolCallEvent toolCall:
                writer.WriteString("name", toolCall.Name);
                writer.WriteString("input", toolCall.Input);
                break;
            case SuspendedEvent suspended:
                writer.WriteString("tool_call_id", suspended.ToolCallId);
                writer.WriteString("tool_name", suspended.ToolName);
                writer.WriteString("arguments_json", suspended.ArgumentsJson);
                writer.WriteString("prompt", suspended.Prompt);
                break;
            default:
                throw new NotSupportedException($"Unknown RunEvent subtype: {value.GetType()}");
        }
        writer.WriteEndObject();
    }
}
