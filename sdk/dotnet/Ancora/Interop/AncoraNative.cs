using System;
using System.Runtime.InteropServices;

namespace Ancora.Interop;

/// <summary>
/// Raw P/Invoke declarations for the ancora-ffi C ABI.
/// All functions use the Cdecl calling convention matching the Rust extern "C" surface.
/// </summary>
internal static unsafe class AncoraNative
{
    private const string Lib = "ancora_ffi";

    // --- Buffer lifecycle ---

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorBuffer ancora_buffer_new(byte* bytes, nuint len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void ancora_buffer_free(AncorBuffer buf);

    // --- Run ID ---

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr ancora_run_id_new(
        [MarshalAs(UnmanagedType.LPUTF8Str)] string s);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void ancora_run_id_free(IntPtr ptr);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorBuffer ancora_run_id_to_str(IntPtr ptr);

    // --- Runtime lifecycle ---

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_runtime_new(out IntPtr outRuntime);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_runtime_new_with_config(
        IntPtr configBytes, nuint configLen, out IntPtr outRuntime);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void ancora_free_runtime(IntPtr ptr);

    // --- Run operations ---

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_run_start(
        IntPtr rt,
        IntPtr specBytes, nuint specLen,
        out AncorBuffer outRunId);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_run_poll(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string runId,
        out AncorBuffer outEvent);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_run_resume(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string runId,
        IntPtr decisionBytes, nuint decisionLen);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_run_cost(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string runId,
        out AncorBuffer outCost);

    // --- Tool registration ---

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_tool_register(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string name,
        AncorToolCallback cb);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_tool_register_requires_approval(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string name,
        AncorToolCallback cb);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_tool_unregister(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string name);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_tool_invoke(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string name,
        IntPtr inputBytes, nuint inputLen,
        out AncorBuffer outResult);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern nuint ancora_tool_count(IntPtr rt);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static extern byte ancora_tool_exists(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string name);

    // --- Memory / vector store ---

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_create_collection(
        IntPtr rt, IntPtr specBytes, nuint specLen);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_drop_collection(
        IntPtr rt, [MarshalAs(UnmanagedType.LPUTF8Str)] string name);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_upsert(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string collection,
        IntPtr pointsBytes, nuint pointsLen);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_query(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string collection,
        IntPtr queryBytes, nuint queryLen,
        out AncorBuffer outResult);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_delete(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string collection,
        IntPtr idsBytes, nuint idsLen);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_delete_by_filter(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string collection,
        IntPtr filterBytes, nuint filterLen,
        out AncorBuffer outResult);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_hybrid_query(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string collection,
        IntPtr queryBytes, nuint queryLen,
        out AncorBuffer outResult);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern AncorErrorCode ancora_memory_describe_collection(
        IntPtr rt,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string name,
        out AncorBuffer outResult);

    // --- Version ---

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr ancora_version();
}

/// <summary>
/// Managed delegate matching the AncorToolCallback C function pointer type.
/// Callers must pin the delegate to prevent GC collection while registered.
/// </summary>
[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal unsafe delegate AncorErrorCode AncorToolCallback(
    byte* input, nuint inputLen, AncorBuffer* outBuf);
