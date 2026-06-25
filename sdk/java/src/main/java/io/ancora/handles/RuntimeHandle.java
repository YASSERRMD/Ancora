package io.ancora.handles;

import io.ancora.AncorException;
import io.ancora.ffi.AncorErrorCode;
import io.ancora.ffi.AncoraNative;

import java.lang.foreign.*;

/**
 * Manages the lifetime of an opaque AncorRuntime pointer.
 * Calls ancora_free_runtime when closed. AutoCloseable for try-with-resources.
 */
public final class RuntimeHandle implements AutoCloseable {

    private final MemorySegment ptr;
    private volatile boolean closed = false;

    private RuntimeHandle(MemorySegment ptr) {
        this.ptr = ptr;
    }

    /**
     * Allocate a new native runtime via ancora_runtime_new.
     *
     * @throws UnsatisfiedLinkError if the native library was not loaded
     * @throws AncorException if ancora_runtime_new returns a non-OK code
     */
    public static RuntimeHandle create() throws Throwable {
        if (!AncoraNative.AVAILABLE) {
            throw new UnsatisfiedLinkError("ancora_ffi native library not found");
        }
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment outPtr = arena.allocate(ValueLayout.ADDRESS);
            int rc = (int) AncoraNative.RUNTIME_NEW.invokeExact(outPtr);
            if (rc != 0) {
                throw new AncorException(rc, "ancora_runtime_new failed");
            }
            MemorySegment runtimePtr = outPtr.get(ValueLayout.ADDRESS, 0);
            if (runtimePtr.equals(MemorySegment.NULL)) {
                throw new AncorException(AncorErrorCode.INTERNAL, "ancora_runtime_new returned null");
            }
            return new RuntimeHandle(runtimePtr.reinterpret(Long.MAX_VALUE));
        }
    }

    /**
     * Return the raw native pointer for passing to FFI functions.
     *
     * @throws IllegalStateException if the handle has been closed
     */
    public MemorySegment rawPtr() {
        if (closed) throw new IllegalStateException("RuntimeHandle has been closed");
        return ptr;
    }

    @Override
    public void close() throws Throwable {
        if (!closed) {
            closed = true;
            AncoraNative.FREE_RUNTIME.invokeExact(ptr);
        }
    }
}
