package io.ancora;

import com.fasterxml.jackson.databind.JsonNode;
import io.ancora.ffi.AncoraNative;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.nio.charset.StandardCharsets;

final class ToolBridge implements AutoCloseable {

    private final Arena arena;
    private final MemorySegment stub;

    private ToolBridge(Arena arena, MemorySegment stub) {
        this.arena = arena;
        this.stub = stub;
    }

    static ToolBridge create(ToolHandler handler) throws Throwable {
        Arena arena = Arena.ofShared();
        MethodHandle mh = MethodHandles.lookup()
            .findStatic(ToolBridge.class, "dispatch",
                MethodType.methodType(
                    int.class, ToolHandler.class,
                    MemorySegment.class, long.class, MemorySegment.class))
            .bindTo(handler);
        MemorySegment stub = AncoraNative.LINKER.upcallStub(
            mh, AncoraNative.TOOL_CALLBACK_DESC, arena);
        return new ToolBridge(arena, stub);
    }

    // Invoked from native code via upcall stub.
    static int dispatch(ToolHandler handler,
                        MemorySegment inputPtr, long inputLen,
                        MemorySegment outBufPtr) {
        try {
            byte[] inputBytes = inputPtr.reinterpret(inputLen).toArray(ValueLayout.JAVA_BYTE);
            JsonNode node = Wire.MAPPER.readTree(inputBytes);
            String result = handler.handle(node);
            if (result == null) result = "";
            byte[] resultBytes = result.getBytes(StandardCharsets.UTF_8);
            try (Arena scratch = Arena.ofConfined()) {
                MemorySegment srcSeg = scratch.allocateFrom(ValueLayout.JAVA_BYTE, resultBytes);
                MemorySegment bufSeg = (MemorySegment) AncoraNative.BUFFER_NEW.invokeExact(
                    scratch, srcSeg, (long) resultBytes.length);
                MemorySegment nativePtr = (MemorySegment) AncoraNative.BUFFER_PTR.get(bufSeg, 0L);
                long nativeLen = (long) AncoraNative.BUFFER_LEN.get(bufSeg, 0L);
                MemorySegment outBuf = outBufPtr.reinterpret(AncoraNative.BUFFER_LAYOUT.byteSize());
                AncoraNative.BUFFER_PTR.set(outBuf, 0L, nativePtr);
                AncoraNative.BUFFER_LEN.set(outBuf, 0L, nativeLen);
            }
            return 0; // AncorErrorCode.OK
        } catch (Throwable t) {
            return 3; // AncorErrorCode.INTERNAL
        }
    }

    MemorySegment stub() {
        return stub;
    }

    @Override
    public void close() {
        arena.close();
    }
}
