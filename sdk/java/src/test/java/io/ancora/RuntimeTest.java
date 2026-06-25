package io.ancora;

import io.ancora.ffi.AncorErrorCode;
import io.ancora.ffi.AncoraNative;
import io.ancora.handles.RuntimeHandle;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class RuntimeTest {

    // --- AncorErrorCode tests (no native lib required) ---

    @Test
    void errorCode_ok_isZero() {
        assertEquals(0, AncorErrorCode.OK.value);
    }

    @Test
    void errorCode_nullPtr_isOne() {
        assertEquals(1, AncorErrorCode.NULL_PTR.value);
    }

    @Test
    void errorCode_invalidUtf8_isTwo() {
        assertEquals(2, AncorErrorCode.INVALID_UTF8.value);
    }

    @Test
    void errorCode_internal_isThree() {
        assertEquals(3, AncorErrorCode.INTERNAL.value);
    }

    @Test
    void errorCode_from_returnsMatchingValue() {
        assertSame(AncorErrorCode.OK, AncorErrorCode.from(0));
        assertSame(AncorErrorCode.NULL_PTR, AncorErrorCode.from(1));
        assertSame(AncorErrorCode.INTERNAL, AncorErrorCode.from(99));
    }

    @Test
    void errorCode_isOk_returnsTrueForOk() {
        assertTrue(AncorErrorCode.OK.isOk());
        assertFalse(AncorErrorCode.INTERNAL.isOk());
    }

    // --- AncorException tests (no native lib required) ---

    @Test
    void ancorException_carriesErrorCode() {
        AncorException ex = new AncorException(3, "failure");
        assertEquals(3, ex.getErrorCode());
        assertTrue(ex.getMessage().contains("failure"));
        assertTrue(ex.getMessage().contains("3"));
    }

    @Test
    void ancorException_fromEnum_setsCorrectCode() {
        AncorException ex = new AncorException(AncorErrorCode.NULL_PTR, "null");
        assertEquals(1, ex.getErrorCode());
    }

    // --- RuntimeHandle type tests (no native lib required) ---

    @Test
    void runtimeHandle_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(RuntimeHandle.class));
    }

    @Test
    void runtime_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Runtime.class));
    }

    // --- Integration tests (require native library) ---

    @Test
    void runtime_createsAndCloses() {
        skipIfNativeLibraryAbsent();
        try (Runtime rt = new Runtime()) {
            assertNotNull(rt);
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void runtime_doubleCloseIsSafe() {
        skipIfNativeLibraryAbsent();
        try {
            Runtime rt = new Runtime();
            rt.close();
            rt.close(); // must not throw
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void runtime_version_returnsString() {
        skipIfNativeLibraryAbsent();
        try {
            String v = Runtime.version();
            assertNotNull(v);
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void runtime_toolCount_isZeroAfterCreate() {
        skipIfNativeLibraryAbsent();
        try (Runtime rt = new Runtime()) {
            assertEquals(0L, rt.toolCount());
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void runtime_toolExists_returnsFalseForUnknown() {
        skipIfNativeLibraryAbsent();
        try (Runtime rt = new Runtime()) {
            assertFalse(rt.toolExists("nonexistent_tool"));
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    // --- helpers ---

    private static void skipIfNativeLibraryAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE,
            "ancora_ffi native library not present; CI provides it.");
    }
}
