package io.ancora;

import io.ancora.ffi.AncorErrorCode;
import io.ancora.ffi.AncoraNative;
import io.ancora.handles.RuntimeHandle;

import java.lang.foreign.*;
import java.nio.charset.StandardCharsets;

/**
 * Managed entry point to the Ancora native runtime.
 * Use try-with-resources to ensure the native runtime is freed.
 */
public final class Runtime implements AutoCloseable {

    private final RuntimeHandle handle;
    private volatile boolean closed = false;

    /**
     * Allocate a new Ancora runtime.
     *
     * @throws UnsatisfiedLinkError if the native library is not loaded
     * @throws AncorException if runtime allocation fails
     */
    public Runtime() throws Throwable {
        this.handle = RuntimeHandle.create();
    }

    /**
     * Return the ABI version string from the native library.
     *
     * @throws UnsatisfiedLinkError if the native library is not loaded
     */
    public static String version() throws Throwable {
        if (!AncoraNative.AVAILABLE) throw new UnsatisfiedLinkError("ancora_ffi not loaded");
        MemorySegment ptr = (MemorySegment) AncoraNative.VERSION.invokeExact();
        if (ptr.equals(MemorySegment.NULL)) return "";
        return ptr.reinterpret(256).getString(0, StandardCharsets.UTF_8);
    }

    /**
     * Start a new run from serialized spec bytes.
     * Returns the run ID string.
     */
    public String startRun(byte[] specBytes) throws Throwable {
        checkOpen();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment specSeg = arena.allocateFrom(ValueLayout.JAVA_BYTE, specBytes);
            MemorySegment outBuf = arena.allocate(AncoraNative.BUFFER_LAYOUT);
            int rc = (int) AncoraNative.RUN_START.invokeExact(
                handle.rawPtr(), specSeg, (long) specBytes.length, outBuf);
            if (rc != 0) throw new AncorException(rc, "ancora_run_start failed");
            return AncoraNative.readBufferAsString(outBuf);
        }
    }

    /**
     * Poll the next event JSON string for a run.
     * Returns null when all events are consumed.
     */
    public String pollEvent(String runId) throws Throwable {
        checkOpen();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment idSeg = arena.allocateFrom(runId);
            MemorySegment outBuf = arena.allocate(AncoraNative.BUFFER_LAYOUT);
            int rc = (int) AncoraNative.RUN_POLL.invokeExact(
                handle.rawPtr(), idSeg, outBuf);
            if (rc != 0) throw new AncorException(rc, "ancora_run_poll failed");
            MemorySegment ptr = (MemorySegment) AncoraNative.BUFFER_PTR.get(outBuf, 0L);
            if (ptr.equals(MemorySegment.NULL)) return null;
            return AncoraNative.readBufferAsString(outBuf);
        }
    }

    /**
     * Resume a suspended run with a decision payload.
     */
    public void resumeRun(String runId, byte[] decisionBytes) throws Throwable {
        checkOpen();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment idSeg = arena.allocateFrom(runId);
            MemorySegment decisionSeg = arena.allocateFrom(ValueLayout.JAVA_BYTE, decisionBytes);
            int rc = (int) AncoraNative.RUN_RESUME.invokeExact(
                handle.rawPtr(), idSeg, decisionSeg, (long) decisionBytes.length);
            if (rc != 0) throw new AncorException(rc, "ancora_run_resume failed");
        }
    }

    /**
     * Return the cost summary JSON for a completed run.
     */
    public String getCost(String runId) throws Throwable {
        checkOpen();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment idSeg = arena.allocateFrom(runId);
            MemorySegment outBuf = arena.allocate(AncoraNative.BUFFER_LAYOUT);
            int rc = (int) AncoraNative.RUN_COST.invokeExact(
                handle.rawPtr(), idSeg, outBuf);
            if (rc != 0) throw new AncorException(rc, "ancora_run_cost failed");
            return AncoraNative.readBufferAsString(outBuf);
        }
    }

    /**
     * Return the number of registered tool callbacks.
     */
    public long toolCount() throws Throwable {
        checkOpen();
        return (long) AncoraNative.TOOL_COUNT.invokeExact(handle.rawPtr());
    }

    /**
     * Return true if a tool with the given name is registered.
     */
    public boolean toolExists(String name) throws Throwable {
        checkOpen();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment nameSeg = arena.allocateFrom(name);
            byte result = (byte) AncoraNative.TOOL_EXISTS.invokeExact(handle.rawPtr(), nameSeg);
            return result != 0;
        }
    }

    MemorySegment rawPtr() {
        checkOpen();
        return handle.rawPtr();
    }

    @Override
    public void close() {
        if (!closed) {
            closed = true;
            handle.close();
        }
    }

    private void checkOpen() {
        if (closed) throw new IllegalStateException("Runtime has been closed");
    }
}
