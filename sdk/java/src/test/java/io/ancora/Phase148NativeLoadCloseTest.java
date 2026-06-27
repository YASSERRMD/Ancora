package io.ancora;

import io.ancora.ffi.AncoraNative;
import io.ancora.handles.RuntimeHandle;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class Phase148NativeLoadCloseTest {

    @Test
    void runtime_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Runtime.class));
    }

    @Test
    void runtime_isFinal() {
        assertTrue(java.lang.reflect.Modifier.isFinal(Runtime.class.getModifiers()));
    }

    @Test
    void runtimeHandle_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(RuntimeHandle.class));
    }

    @Test
    void runtime_createAndClose() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            assertNotNull(rt);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtime_doubleCloseIsSafe() {
        skipIfAbsent();
        try {
            Runtime rt = new Runtime();
            rt.close();
            rt.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtime_closeMarksAsClosed() {
        skipIfAbsent();
        try {
            Runtime rt = new Runtime();
            rt.close();
            assertThrows(IllegalStateException.class, () -> {
                try { rt.toolCount(); } catch (Throwable inner) {
                    if (inner instanceof IllegalStateException e) throw e;
                    throw new RuntimeException(inner);
                }
            });
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtime_versionReturnsString() {
        skipIfAbsent();
        try {
            String v = Runtime.version();
            assertNotNull(v);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtime_toolCountZeroAfterCreate() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            assertEquals(0L, rt.toolCount());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtime_toolExistsReturnsFalseForUnknown() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            assertFalse(rt.toolExists("ghost_tool"));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agent_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Agent.class));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
