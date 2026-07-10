using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using Ancora.Handles;
using Ancora.Interop;

namespace Ancora;

/// <summary>
/// Managed wrapper around the Ancora native runtime.
/// Dispose this instance to release the underlying native memory.
/// </summary>
public sealed class Runtime : IDisposable
{
    private readonly RuntimeHandle _handle;
    private bool _disposed;

    /// <summary>
    /// Create a new Ancora runtime with default configuration.
    /// </summary>
    /// <exception cref="AncorException">Thrown if the native runtime cannot be allocated.</exception>
    /// <exception cref="DllNotFoundException">Thrown if the native library is not found.</exception>
    public Runtime()
    {
        _handle = RuntimeHandle.Create();
    }

    /// <summary>
    /// Create a new Ancora runtime pointed at a real OpenAI-compatible
    /// provider (hosted or self-hosted, including NVIDIA NIM) instead of
    /// the offline echo model client the parameterless constructor uses.
    /// </summary>
    /// <exception cref="AncorException">Thrown if the native runtime cannot be allocated.</exception>
    /// <exception cref="DllNotFoundException">Thrown if the native library is not found.</exception>
    public Runtime(ProviderConfig provider)
    {
        ArgumentNullException.ThrowIfNull(provider);
        var bytes = Wire.EncodeRuntimeConfig(provider);
        _handle = RuntimeHandle.Create(bytes);
    }

    /// <summary>
    /// Create a new Ancora runtime pointed at a real Postgres + pgvector
    /// instance for document embeddings instead of the zero-dependency
    /// in-memory vector store the parameterless constructor uses.
    /// </summary>
    /// <exception cref="AncorException">Thrown if the native runtime cannot be allocated.</exception>
    /// <exception cref="DllNotFoundException">Thrown if the native library is not found.</exception>
    public Runtime(MemoryConfig memory) : this(null, memory)
    {
    }

    /// <summary>
    /// Create a new Ancora runtime with a real model provider, a real
    /// pgvector-backed memory store, or both. Either may be omitted (null)
    /// to keep that half's zero-dependency default (offline echo model
    /// client, or in-memory vector store, respectively).
    /// </summary>
    /// <exception cref="AncorException">Thrown if the native runtime cannot be allocated.</exception>
    /// <exception cref="DllNotFoundException">Thrown if the native library is not found.</exception>
    public Runtime(ProviderConfig? provider, MemoryConfig? memory)
    {
        var bytes = Wire.EncodeRuntimeConfig(provider, memory);
        _handle = RuntimeHandle.Create(bytes);
    }

    /// <summary>
    /// Return the ABI version string from the native library.
    /// </summary>
    public static string Version()
    {
        var ptr = AncoraNative.ancora_version();
        return ptr == IntPtr.Zero ? string.Empty : Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
    }

    /// <summary>
    /// Start a new run from a serialized agent spec.
    /// Returns the run ID string.
    /// </summary>
    public unsafe string StartRun(ReadOnlySpan<byte> specBytes)
    {
        ThrowIfDisposed();
        fixed (byte* p = specBytes)
        {
            var rc = AncoraNative.ancora_run_start(
                _handle.DangerousGetHandle(),
                (IntPtr)p, (nuint)specBytes.Length,
                out var buf);
            if (rc != AncorErrorCode.Ok)
                throw new AncorException((int)rc, "ancora_run_start failed");
            try
            {
                return buf.IsEmpty ? string.Empty : Encoding.UTF8.GetString(buf.AsSpan());
            }
            finally
            {
                AncoraNative.ancora_buffer_free(buf);
            }
        }
    }

    /// <summary>
    /// Poll the next event JSON for a run.
    /// Returns null when all events are consumed.
    /// </summary>
    public string? PollEvent(string runId)
    {
        ThrowIfDisposed();
        var rc = AncoraNative.ancora_run_poll(
            _handle.DangerousGetHandle(), runId, out var buf);
        if (rc != AncorErrorCode.Ok)
            throw new AncorException((int)rc, "ancora_run_poll failed");
        if (buf.IsEmpty) return null;
        try
        {
            return Encoding.UTF8.GetString(buf.AsSpan());
        }
        finally
        {
            AncoraNative.ancora_buffer_free(buf);
        }
    }

    /// <summary>
    /// Resume a suspended run with a decision payload.
    /// </summary>
    public unsafe void ResumeRun(string runId, ReadOnlySpan<byte> decisionBytes)
    {
        ThrowIfDisposed();
        fixed (byte* p = decisionBytes)
        {
            var rc = AncoraNative.ancora_run_resume(
                _handle.DangerousGetHandle(),
                runId, (IntPtr)p, (nuint)decisionBytes.Length);
            if (rc != AncorErrorCode.Ok)
                throw new AncorException((int)rc, "ancora_run_resume failed");
        }
    }

    /// <summary>
    /// Resume a suspended run with a typed tool-call decision: the JSON
    /// value the pending gated tool call should have returned, and whether
    /// it represents an error. Use this instead of the plain-text
    /// <see cref="ResumeRun(string, ReadOnlySpan{byte})"/> overload when the
    /// pending call needs a structured result rather than a bare string.
    /// </summary>
    public void ResumeRun(string runId, string resultJson, bool isError = false)
    {
        ArgumentNullException.ThrowIfNull(resultJson);
        ResumeRun(runId, Wire.EncodeToolDecision(resultJson, isError));
    }

    /// <summary>
    /// Return the cost summary JSON for a completed run.
    /// </summary>
    public string GetCost(string runId)
    {
        ThrowIfDisposed();
        var rc = AncoraNative.ancora_run_cost(
            _handle.DangerousGetHandle(), runId, out var buf);
        if (rc != AncorErrorCode.Ok)
            throw new AncorException((int)rc, "ancora_run_cost failed");
        if (buf.IsEmpty) return "{}";
        try
        {
            return Encoding.UTF8.GetString(buf.AsSpan());
        }
        finally
        {
            AncoraNative.ancora_buffer_free(buf);
        }
    }

    /// <summary>
    /// Return the cost summary for a completed run, parsed into a typed
    /// <see cref="Cost"/> record.
    /// </summary>
    public Cost GetCostTyped(string runId)
    {
        var json = GetCost(runId);
        return System.Text.Json.JsonSerializer.Deserialize<Cost>(json, Wire.Options)
            ?? new Cost(runId, 0.0);
    }

    /// <summary>
    /// Create a vector collection for document embeddings.
    /// </summary>
    /// <param name="name">Collection name.</param>
    /// <param name="dimensions">Embedding dimensionality every point in this collection must match.</param>
    /// <param name="distance">One of <c>"cosine"</c>, <c>"dot"</c>, or <c>"l2"</c>.</param>
    public unsafe void CreateCollection(string name, int dimensions, string distance = "cosine")
    {
        ThrowIfDisposed();
        var bytes = Wire.EncodeCollectionSpec(name, dimensions, distance);
        fixed (byte* p = bytes)
        {
            var rc = AncoraNative.ancora_memory_create_collection(
                _handle.DangerousGetHandle(), (IntPtr)p, (nuint)bytes.Length);
            if (rc != AncorErrorCode.Ok)
            {
                throw new AncorException(
                    (int)rc, $"ancora_memory_create_collection failed for '{name}'");
            }
        }
    }

    /// <summary>
    /// Drop a vector collection by name.
    /// </summary>
    public void DropCollection(string name)
    {
        ThrowIfDisposed();
        var rc = AncoraNative.ancora_memory_drop_collection(_handle.DangerousGetHandle(), name);
        if (rc != AncorErrorCode.Ok)
            throw new AncorException((int)rc, $"ancora_memory_drop_collection failed for '{name}'");
    }

    /// <summary>
    /// Upsert points into a collection.
    /// </summary>
    public unsafe void Upsert(string collection, IReadOnlyList<VectorPoint> points)
    {
        ThrowIfDisposed();
        var bytes = Wire.EncodePoints(points);
        fixed (byte* p = bytes)
        {
            var rc = AncoraNative.ancora_memory_upsert(
                _handle.DangerousGetHandle(), collection, (IntPtr)p, (nuint)bytes.Length);
            if (rc != AncorErrorCode.Ok)
                throw new AncorException((int)rc, $"ancora_memory_upsert failed for '{collection}'");
        }
    }

    /// <summary>
    /// Run a similarity query against a collection.
    /// </summary>
    /// <param name="collection">Collection to query.</param>
    /// <param name="vector">Query embedding vector.</param>
    /// <param name="topK">Maximum number of results to return.</param>
    /// <param name="scoreThreshold">Drop results scoring below this threshold, if set.</param>
    /// <param name="filter">Scope results to points matching this filter, if set.</param>
    public unsafe IReadOnlyList<ScoredVectorPoint> Query(
        string collection,
        float[] vector,
        int topK = 10,
        double? scoreThreshold = null,
        VectorFilter? filter = null)
    {
        ThrowIfDisposed();
        var bytes = Wire.EncodeQueryRequest(vector, topK, scoreThreshold, filter);
        fixed (byte* p = bytes)
        {
            var rc = AncoraNative.ancora_memory_query(
                _handle.DangerousGetHandle(), collection, (IntPtr)p, (nuint)bytes.Length, out var buf);
            if (rc != AncorErrorCode.Ok)
                throw new AncorException((int)rc, $"ancora_memory_query failed for '{collection}'");
            try
            {
                return Wire.ParseScoredPoints(buf.AsSpan());
            }
            finally
            {
                AncoraNative.ancora_buffer_free(buf);
            }
        }
    }

    /// <summary>
    /// Run a hybrid (dense-vector + keyword) similarity query against a
    /// collection, blending the two scores by <paramref name="alpha"/>
    /// (1.0 = pure vector, 0.0 = pure keyword).
    /// </summary>
    public unsafe IReadOnlyList<ScoredVectorPoint> HybridQuery(
        string collection,
        float[] denseVector,
        string keyword,
        int topK = 10,
        float alpha = 0.5f,
        double? scoreThreshold = null,
        VectorFilter? filter = null)
    {
        ThrowIfDisposed();
        var bytes = Wire.EncodeHybridQueryRequest(
            denseVector, keyword, topK, alpha, scoreThreshold, filter);
        fixed (byte* p = bytes)
        {
            var rc = AncoraNative.ancora_memory_hybrid_query(
                _handle.DangerousGetHandle(), collection, (IntPtr)p, (nuint)bytes.Length, out var buf);
            if (rc != AncorErrorCode.Ok)
                throw new AncorException((int)rc, $"ancora_memory_hybrid_query failed for '{collection}'");
            try
            {
                return Wire.ParseScoredPoints(buf.AsSpan());
            }
            finally
            {
                AncoraNative.ancora_buffer_free(buf);
            }
        }
    }

    /// <summary>
    /// Describe a collection: dimensions, point count, and distance metric.
    /// </summary>
    public CollectionInfo DescribeCollection(string collection)
    {
        ThrowIfDisposed();
        var rc = AncoraNative.ancora_memory_describe_collection(
            _handle.DangerousGetHandle(), collection, out var buf);
        if (rc != AncorErrorCode.Ok)
        {
            throw new AncorException(
                (int)rc, $"ancora_memory_describe_collection failed for '{collection}'");
        }
        try
        {
            return Wire.ParseCollectionInfo(buf.AsSpan());
        }
        finally
        {
            AncoraNative.ancora_buffer_free(buf);
        }
    }

    /// <summary>
    /// Delete points from a collection by id.
    /// </summary>
    public unsafe void Delete(string collection, IEnumerable<ulong> ids)
    {
        ThrowIfDisposed();
        var bytes = Wire.EncodeIds(ids);
        fixed (byte* p = bytes)
        {
            var rc = AncoraNative.ancora_memory_delete(
                _handle.DangerousGetHandle(), collection, (IntPtr)p, (nuint)bytes.Length);
            if (rc != AncorErrorCode.Ok)
                throw new AncorException((int)rc, $"ancora_memory_delete failed for '{collection}'");
        }
    }

    /// <summary>
    /// Delete every point matching a filter expression. Returns the number
    /// of points deleted.
    /// </summary>
    public unsafe ulong DeleteByFilter(string collection, VectorFilter filter)
    {
        ThrowIfDisposed();
        ArgumentNullException.ThrowIfNull(filter);
        var bytes = Wire.EncodeFilterBytes(filter);
        fixed (byte* p = bytes)
        {
            var rc = AncoraNative.ancora_memory_delete_by_filter(
                _handle.DangerousGetHandle(), collection, (IntPtr)p, (nuint)bytes.Length, out var buf);
            if (rc != AncorErrorCode.Ok)
            {
                throw new AncorException(
                    (int)rc, $"ancora_memory_delete_by_filter failed for '{collection}'");
            }
            try
            {
                return Wire.ParseDeletedCount(buf.AsSpan());
            }
            finally
            {
                AncoraNative.ancora_buffer_free(buf);
            }
        }
    }

    /// <summary>
    /// Return the number of registered tool callbacks.
    /// </summary>
    public nuint ToolCount()
    {
        ThrowIfDisposed();
        return AncoraNative.ancora_tool_count(_handle.DangerousGetHandle());
    }

    /// <summary>
    /// Return true if a tool with the given name is registered.
    /// </summary>
    public bool ToolExists(string name)
    {
        ThrowIfDisposed();
        return AncoraNative.ancora_tool_exists(_handle.DangerousGetHandle(), name) != 0;
    }

    public void Dispose()
    {
        if (_disposed) return;
        _handle.Dispose();
        _disposed = true;
    }

    /// <summary>
    /// Register a native callback under the given name.
    /// The delegate must stay alive as long as it is registered.
    /// </summary>
    internal void RegisterCallback(string name, AncorToolCallback cb)
    {
        ThrowIfDisposed();
        var rc = AncoraNative.ancora_tool_register(_handle.DangerousGetHandle(), name, cb);
        if (rc != AncorErrorCode.Ok)
            throw new AncorException((int)rc, $"ancora_tool_register failed for '{name}'");
    }

    /// <summary>
    /// Register a native callback that requires human approval before every
    /// call: the run pauses at a <see cref="SuspendedEvent"/> instead of
    /// invoking <paramref name="cb"/>, and stays paused until
    /// <see cref="RunHandle.Resume(string, bool)"/>/<see cref="ResumeRun(string, string, bool)"/>
    /// supplies a decision for that tool call. The delegate must stay alive
    /// as long as it is registered.
    /// </summary>
    internal void RegisterCallbackRequiringApproval(string name, AncorToolCallback cb)
    {
        ThrowIfDisposed();
        var rc = AncoraNative.ancora_tool_register_requires_approval(
            _handle.DangerousGetHandle(), name, cb);
        if (rc != AncorErrorCode.Ok)
        {
            throw new AncorException(
                (int)rc, $"ancora_tool_register_requires_approval failed for '{name}'");
        }
    }

    /// <summary>
    /// Unregister a previously registered callback by name.
    /// </summary>
    internal void UnregisterCallback(string name)
    {
        if (_disposed) return;
        AncoraNative.ancora_tool_unregister(_handle.DangerousGetHandle(), name);
    }

    private void ThrowIfDisposed()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
    }
}
