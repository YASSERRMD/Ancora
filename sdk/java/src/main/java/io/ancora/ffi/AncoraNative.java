package io.ancora.ffi;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;
import java.nio.charset.StandardCharsets;

/**
 * Raw FFM bindings for the ancora-ffi C ABI.
 * Uses the Foreign Function and Memory API (stable since Java 22).
 * Loads libancora_ffi at class initialization; AVAILABLE is false
 * when the library is absent so callers can skip FFI-dependent paths.
 */
public final class AncoraNative {

    private AncoraNative() {}

    // --- AncorBuffer struct layout ---

    /** Sequential layout matching: uint8_t *ptr + uintptr_t len (64-bit). */
    public static final StructLayout BUFFER_LAYOUT = MemoryLayout.structLayout(
        ValueLayout.ADDRESS.withName("ptr"),
        ValueLayout.JAVA_LONG.withName("len")
    ).withName("AncorBuffer");

    public static final VarHandle BUFFER_PTR =
        BUFFER_LAYOUT.varHandle(MemoryLayout.PathElement.groupElement("ptr"));
    public static final VarHandle BUFFER_LEN =
        BUFFER_LAYOUT.varHandle(MemoryLayout.PathElement.groupElement("len"));

    // --- Callback descriptor (for upcall stubs) ---

    /** FunctionDescriptor matching AncorToolCallback in the C ABI. */
    public static final FunctionDescriptor TOOL_CALLBACK_DESC = FunctionDescriptor.of(
        ValueLayout.JAVA_INT,
        ValueLayout.ADDRESS,
        ValueLayout.JAVA_LONG,
        ValueLayout.ADDRESS
    );

    // --- Library load ---

    public static final Linker LINKER = Linker.nativeLinker();

    /** True when ancora_ffi was loaded successfully. */
    public static final boolean AVAILABLE;

    private static SymbolLookup LOOKUP;

    // --- Method handles (null when library not found) ---

    public static final MethodHandle RUNTIME_NEW;
    public static final MethodHandle FREE_RUNTIME;
    public static final MethodHandle RUN_ID_NEW;
    public static final MethodHandle RUN_ID_FREE;
    public static final MethodHandle RUN_ID_TO_STR;
    public static final MethodHandle BUFFER_FREE;
    public static final MethodHandle BUFFER_NEW;
    public static final MethodHandle VERSION;
    public static final MethodHandle TOOL_COUNT;
    public static final MethodHandle TOOL_EXISTS;
    public static final MethodHandle RUN_START;
    public static final MethodHandle RUN_POLL;
    public static final MethodHandle RUN_RESUME;
    public static final MethodHandle RUN_COST;
    public static final MethodHandle TOOL_REGISTER;
    public static final MethodHandle TOOL_UNREGISTER;

    static {
        boolean available = false;
        SymbolLookup lookup = null;
        try {
            System.loadLibrary("ancora_ffi");
            lookup = SymbolLookup.loaderLookup();
            available = true;
        } catch (UnsatisfiedLinkError ignored) {}

        AVAILABLE = available;
        LOOKUP = lookup;

        if (available) {
            RUNTIME_NEW = handle("ancora_runtime_new",
                FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));
            FREE_RUNTIME = handle("ancora_free_runtime",
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
            RUN_ID_NEW = handle("ancora_run_id_new",
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            RUN_ID_FREE = handle("ancora_run_id_free",
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
            RUN_ID_TO_STR = handle("ancora_run_id_to_str",
                FunctionDescriptor.of(BUFFER_LAYOUT, ValueLayout.ADDRESS));
            BUFFER_FREE = handle("ancora_buffer_free",
                FunctionDescriptor.ofVoid(BUFFER_LAYOUT));
            BUFFER_NEW = handle("ancora_buffer_new",
                FunctionDescriptor.of(BUFFER_LAYOUT, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));
            VERSION = handle("ancora_version",
                FunctionDescriptor.of(ValueLayout.ADDRESS));
            TOOL_COUNT = handle("ancora_tool_count",
                FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS));
            TOOL_EXISTS = handle("ancora_tool_exists",
                FunctionDescriptor.of(ValueLayout.JAVA_BYTE, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            RUN_START = handle("ancora_run_start",
                FunctionDescriptor.of(ValueLayout.JAVA_INT,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG, ValueLayout.ADDRESS));
            RUN_POLL = handle("ancora_run_poll",
                FunctionDescriptor.of(ValueLayout.JAVA_INT,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            RUN_RESUME = handle("ancora_run_resume",
                FunctionDescriptor.of(ValueLayout.JAVA_INT,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));
            RUN_COST = handle("ancora_run_cost",
                FunctionDescriptor.of(ValueLayout.JAVA_INT,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            TOOL_REGISTER = handle("ancora_tool_register",
                FunctionDescriptor.of(ValueLayout.JAVA_INT,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            TOOL_UNREGISTER = handle("ancora_tool_unregister",
                FunctionDescriptor.of(ValueLayout.JAVA_INT,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS));
        } else {
            RUNTIME_NEW = null; FREE_RUNTIME = null;
            RUN_ID_NEW = null; RUN_ID_FREE = null; RUN_ID_TO_STR = null;
            BUFFER_FREE = null; BUFFER_NEW = null; VERSION = null;
            TOOL_COUNT = null; TOOL_EXISTS = null;
            RUN_START = null; RUN_POLL = null; RUN_RESUME = null; RUN_COST = null;
            TOOL_REGISTER = null; TOOL_UNREGISTER = null;
        }
    }

    private static MethodHandle handle(String name, FunctionDescriptor desc) {
        return LINKER.downcallHandle(LOOKUP.find(name).orElseThrow(
            () -> new IllegalStateException("Symbol not found: " + name)), desc);
    }

    // --- Buffer helpers ---

    /**
     * Read an AncorBuffer struct from a MemorySegment and return the UTF-8 string.
     * Frees the buffer via ancora_buffer_free after reading.
     */
    public static String readBufferAsString(MemorySegment bufSeg) throws Throwable {
        MemorySegment ptr = (MemorySegment) BUFFER_PTR.get(bufSeg, 0L);
        long len = (long) BUFFER_LEN.get(bufSeg, 0L);
        if (ptr.equals(MemorySegment.NULL) || len == 0) return "";
        byte[] bytes = ptr.reinterpret(len).toArray(ValueLayout.JAVA_BYTE);
        String result = new String(bytes, StandardCharsets.UTF_8);
        freeBuffer(bufSeg);
        return result;
    }

    /** Call ancora_buffer_free on the struct segment. */
    public static void freeBuffer(MemorySegment bufSeg) throws Throwable {
        if (BUFFER_FREE != null) {
            MemorySegment ptr = (MemorySegment) BUFFER_PTR.get(bufSeg, 0L);
            long len = (long) BUFFER_LEN.get(bufSeg, 0L);
            if (!ptr.equals(MemorySegment.NULL) && len > 0) {
                BUFFER_FREE.invokeExact(bufSeg);
            }
        }
    }
}
