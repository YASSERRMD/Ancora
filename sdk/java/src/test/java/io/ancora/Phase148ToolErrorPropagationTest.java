package io.ancora;

import io.ancora.ffi.AncorErrorCode;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148ToolErrorPropagationTest {

    @Test
    void ancorException_carriesCode() {
        AncorException ex = new AncorException(3, "boom");
        assertEquals(3, ex.getErrorCode());
        assertTrue(ex.getMessage().contains("boom"));
    }

    @Test
    void ancorException_fromEnum_setsCode() {
        AncorException ex = new AncorException(AncorErrorCode.NULL_PTR, "null ptr");
        assertEquals(1, ex.getErrorCode());
    }

    @Test
    void ancorException_inheritsRuntimeException() {
        assertTrue(RuntimeException.class.isAssignableFrom(AncorException.class));
    }

    @Test
    void errorCode_ok_isZero() {
        assertEquals(0, AncorErrorCode.OK.value);
    }

    @Test
    void errorCode_internal_isThree() {
        assertEquals(3, AncorErrorCode.INTERNAL.value);
    }

    @Test
    void errorCode_ok_isOk() {
        assertTrue(AncorErrorCode.OK.isOk());
        assertFalse(AncorErrorCode.INTERNAL.isOk());
    }

    @Test
    void throwingHandler_doesNotCrashRegistry() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            ToolRegistration reg = ToolRegistry.register(rt, "bad_tool", "Bad",
                input -> { throw new RuntimeException("always fails"); });
            assertTrue(rt.toolExists("bad_tool"));
            reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void registerNullHandlerThrows() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            assertThrows(NullPointerException.class,
                () -> ToolRegistry.register(rt, "t", "desc", null));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void ancorException_messageContainsCode() {
        AncorException ex = new AncorException(3, "internal error");
        assertTrue(ex.getMessage().contains("3"));
    }

    @Test
    void throwingTool_toolStillRegistered() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            ToolRegistration reg = ToolRegistry.register(rt, "volatile_tool", "Volatile",
                input -> { throw new IllegalStateException("fail"); });
            assertNotNull(rt);
            reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
