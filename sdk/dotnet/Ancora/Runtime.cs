using System;
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

    private void ThrowIfDisposed()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
    }
}
