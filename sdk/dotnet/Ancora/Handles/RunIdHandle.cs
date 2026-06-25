using System;
using System.Runtime.InteropServices;
using System.Text;
using Ancora.Interop;

namespace Ancora.Handles;

/// <summary>
/// SafeHandle wrapping an opaque AncorRunId pointer.
/// Calls ancora_run_id_free on finalization or explicit Dispose.
/// </summary>
internal sealed class RunIdHandle : SafeHandle
{
    private RunIdHandle() : base(IntPtr.Zero, ownsHandle: true) { }

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        AncoraNative.ancora_run_id_free(handle);
        return true;
    }

    internal static RunIdHandle FromString(string id)
    {
        var ptr = AncoraNative.ancora_run_id_new(id);
        if (ptr == IntPtr.Zero)
            throw new AncorException((int)AncorErrorCode.InvalidUtf8,
                "ancora_run_id_new returned null");
        var h = new RunIdHandle();
        h.SetHandle(ptr);
        return h;
    }

    internal unsafe string ToRunId()
    {
        var buf = AncoraNative.ancora_run_id_to_str(handle);
        if (buf.IsEmpty) return string.Empty;
        try
        {
            return Encoding.UTF8.GetString(buf.AsSpan());
        }
        finally
        {
            AncoraNative.ancora_buffer_free(buf);
        }
    }
}
