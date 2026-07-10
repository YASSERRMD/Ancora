using System;
using System.Runtime.InteropServices;
using Ancora.Interop;

namespace Ancora.Handles;

/// <summary>
/// SafeHandle wrapping an opaque AncorRuntime pointer.
/// Calls ancora_free_runtime on finalization or explicit Dispose.
/// </summary>
internal sealed class RuntimeHandle : SafeHandle
{
    private RuntimeHandle() : base(IntPtr.Zero, ownsHandle: true) { }

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        AncoraNative.ancora_free_runtime(handle);
        return true;
    }

    internal static RuntimeHandle Create()
    {
        var rc = AncoraNative.ancora_runtime_new(out var ptr);
        if (rc != AncorErrorCode.Ok)
            throw new AncorException((int)rc, $"ancora_runtime_new returned {rc}");
        if (ptr == IntPtr.Zero)
            throw new AncorException((int)AncorErrorCode.Internal, "ancora_runtime_new returned null");
        var h = new RuntimeHandle();
        h.SetHandle(ptr);
        return h;
    }

    internal static unsafe RuntimeHandle Create(ReadOnlySpan<byte> configBytes)
    {
        AncorErrorCode rc;
        IntPtr ptr;
        fixed (byte* p = configBytes)
        {
            rc = AncoraNative.ancora_runtime_new_with_config(
                (IntPtr)p, (nuint)configBytes.Length, out ptr);
        }
        if (rc != AncorErrorCode.Ok)
            throw new AncorException((int)rc, $"ancora_runtime_new_with_config returned {rc}");
        if (ptr == IntPtr.Zero)
        {
            throw new AncorException(
                (int)AncorErrorCode.Internal, "ancora_runtime_new_with_config returned null");
        }
        var h = new RuntimeHandle();
        h.SetHandle(ptr);
        return h;
    }
}
