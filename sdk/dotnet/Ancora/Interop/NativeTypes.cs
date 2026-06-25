using System.Runtime.InteropServices;

namespace Ancora.Interop;

/// <summary>
/// Error codes returned by all Ancora FFI functions.
/// </summary>
internal enum AncorErrorCode : int
{
    Ok = 0,
    NullPtr = 1,
    InvalidUtf8 = 2,
    Internal = 3,
}

/// <summary>
/// Owned byte buffer passed across the FFI boundary.
/// Must be freed with <see cref="AncoraNative.ancora_buffer_free"/> after use.
/// Use <see cref="IntPtr.Zero"/> check on Ptr to detect empty buffers.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
internal struct AncorBuffer
{
    public IntPtr Ptr;
    public nuint Len;

    public readonly bool IsEmpty => Ptr == IntPtr.Zero || Len == 0;

    public readonly unsafe ReadOnlySpan<byte> AsSpan() =>
        Ptr == IntPtr.Zero ? ReadOnlySpan<byte>.Empty
                           : new ReadOnlySpan<byte>((byte*)Ptr, (int)Len);
}
