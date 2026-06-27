using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;

namespace Ancora.Tests.Examples;

/// <summary>
/// Shared helper types used across offline example tests.
/// No native library required -- the offline runtime produces events in-process.
/// </summary>

/// <summary>
/// An in-process run journal that records run IDs and event payloads.
/// Mirrors the role of a SQLite or Redis store in production.
/// </summary>
public sealed class RunJournal
{
    private readonly Dictionary<string, List<string>> _runs = new();

    public void RecordRun(string runId)
    {
        if (!_runs.ContainsKey(runId))
            _runs[runId] = new List<string>();
    }

    public void AppendEvent(string runId, string payload)
    {
        if (_runs.TryGetValue(runId, out var list))
            list.Add(payload);
    }

    public IReadOnlyList<string> EventsForRun(string runId)
        => _runs.TryGetValue(runId, out var list) ? list : Array.Empty<string>();

    public int RunCount => _runs.Count;
}

/// <summary>
/// Minimal stand-in for an OTEL span, to avoid importing OTEL packages.
/// </summary>
public sealed class Span
{
    private readonly long _startedAt = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
    public string Name { get; }
    public Dictionary<string, object> Attributes { get; } = new();

    public Span(string name) => Name = name;

    public void SetAttribute(string key, object value) => Attributes[key] = value;

    public long EndMs() => DateTimeOffset.UtcNow.ToUnixTimeMilliseconds() - _startedAt;
}

/// <summary>
/// Token estimation heuristic: 4 characters per token.
/// </summary>
public static class TokenEstimator
{
    public static int EstimateTokens(string text)
        => Math.Max(1, (int)Math.Ceiling(text.Length / 4.0));
}
