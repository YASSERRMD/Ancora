package io.ancora;

import io.ancora.ffi.AncoraNative;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

final class ToolDisposable implements AutoCloseable {

    private final Runtime runtime;
    private final String name;
    private final ToolBridge bridge;
    private volatile boolean closed = false;

    ToolDisposable(Runtime runtime, String name, ToolBridge bridge) {
        this.runtime = runtime;
        this.name = name;
        this.bridge = bridge;
    }

    @Override
    public void close() {
        if (!closed) {
            closed = true;
            try (Arena scratch = Arena.ofConfined()) {
                MemorySegment nameSeg = scratch.allocateFrom(name);
                int rc;
                try {
                    rc = (int) AncoraNative.TOOL_UNREGISTER.invokeExact(runtime.rawPtr(), nameSeg);
                } catch (Throwable t) {
                    throw new RuntimeException("ancora_tool_unregister failed", t);
                }
                if (rc != 0) {
                    throw new AncorException(rc, "ancora_tool_unregister failed for: " + name);
                }
            } finally {
                bridge.close();
            }
        }
    }
}
