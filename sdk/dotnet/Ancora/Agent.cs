using System;

namespace Ancora;

/// <summary>
/// High-level entry point for running agents.
/// Creates or wraps a <see cref="Runtime"/> and exposes
/// idiomatic .NET start methods.
/// </summary>
public sealed class Agent : IDisposable
{
    private readonly Runtime _runtime;
    private readonly bool _ownsRuntime;
    private bool _disposed;

    /// <summary>
    /// Create an Agent backed by a freshly allocated runtime.
    /// The runtime is disposed when this Agent is disposed.
    /// </summary>
    public Agent()
    {
        _runtime = new Runtime();
        _ownsRuntime = true;
    }

    /// <summary>
    /// Create an Agent that shares an existing runtime.
    /// The caller remains responsible for disposing the runtime.
    /// </summary>
    public Agent(Runtime runtime)
    {
        _runtime = runtime ?? throw new ArgumentNullException(nameof(runtime));
        _ownsRuntime = false;
    }

    /// <summary>
    /// Start a new run from an AgentSpec.
    /// Returns a RunHandle for polling events and resuming.
    /// </summary>
    public RunHandle Run(AgentSpec spec)
    {
        ThrowIfDisposed();
        ArgumentNullException.ThrowIfNull(spec);
        var bytes = Wire.EncodeAgentSpec(spec);
        var runId = _runtime.StartRun(bytes);
        return new RunHandle(runId, _runtime);
    }

    /// <summary>
    /// Start a new run from a GraphSpec.
    /// Returns a RunHandle for polling events and resuming.
    /// </summary>
    public RunHandle RunGraph(GraphSpec graph)
    {
        ThrowIfDisposed();
        ArgumentNullException.ThrowIfNull(graph);
        var bytes = Wire.EncodeGraphSpec(graph);
        var runId = _runtime.StartRun(bytes);
        return new RunHandle(runId, _runtime);
    }

    public void Dispose()
    {
        if (_disposed) return;
        if (_ownsRuntime) _runtime.Dispose();
        _disposed = true;
    }

    private void ThrowIfDisposed() =>
        ObjectDisposedException.ThrowIf(_disposed, this);
}
