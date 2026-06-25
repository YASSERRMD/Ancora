using System;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Ancora;

/// <summary>
/// A handle to a live or completed run.
/// Provides an IAsyncEnumerable event stream and resume support.
/// </summary>
public sealed class RunHandle
{
    /// <summary>
    /// The stable run ID assigned when the run was created.
    /// </summary>
    public string RunId { get; }

    private readonly Runtime _runtime;

    internal RunHandle(string runId, Runtime runtime)
    {
        RunId = runId ?? throw new ArgumentNullException(nameof(runId));
        _runtime = runtime ?? throw new ArgumentNullException(nameof(runtime));
    }

    /// <summary>
    /// Stream events from this run as an async sequence.
    /// The stream ends when the run produces no further events.
    /// </summary>
    public async IAsyncEnumerable<RunEvent> EventsAsync(
        [EnumeratorCancellation] CancellationToken cancellationToken = default)
    {
        while (!cancellationToken.IsCancellationRequested)
        {
            var raw = _runtime.PollEvent(RunId);
            if (raw is null) yield break;
            yield return Wire.ParseEvent(raw);
            await Task.Yield();
        }
    }

    /// <summary>
    /// Collect all remaining events from this run into a list.
    /// Returns after the run produces no further events.
    /// </summary>
    public async Task<IReadOnlyList<RunEvent>> CollectAsync(
        CancellationToken cancellationToken = default)
    {
        var events = new List<RunEvent>();
        await foreach (var ev in EventsAsync(cancellationToken).WithCancellation(cancellationToken))
            events.Add(ev);
        return events;
    }

    /// <summary>
    /// Resume a suspended run, injecting a decision payload as a UTF-8 string.
    /// </summary>
    public void Resume(string decision)
    {
        ArgumentNullException.ThrowIfNull(decision);
        var bytes = Wire.EncodeDecision(decision);
        _runtime.ResumeRun(RunId, bytes);
    }

    /// <summary>
    /// Resume a suspended run with a raw byte decision payload.
    /// </summary>
    public void Resume(ReadOnlySpan<byte> decisionBytes) =>
        _runtime.ResumeRun(RunId, decisionBytes);

    /// <summary>
    /// Resume a suspended run and then collect all subsequent events.
    /// </summary>
    public async Task<IReadOnlyList<RunEvent>> ResumeAndCollectAsync(
        string decision,
        CancellationToken cancellationToken = default)
    {
        Resume(decision);
        return await CollectAsync(cancellationToken);
    }

    /// <summary>
    /// Return the cost summary JSON for this run.
    /// </summary>
    public string GetCost() => _runtime.GetCost(RunId);
}
