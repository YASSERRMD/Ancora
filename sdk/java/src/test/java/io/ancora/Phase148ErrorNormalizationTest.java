package io.ancora;

import io.ancora.ffi.AncorErrorCode;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class Phase148ErrorNormalizationTest {

    @Test
    void errorCode_ok_value_isZero() {
        assertEquals(0, AncorErrorCode.OK.value);
    }

    @Test
    void errorCode_nullPtr_value_isOne() {
        assertEquals(1, AncorErrorCode.NULL_PTR.value);
    }

    @Test
    void errorCode_invalidUtf8_value_isTwo() {
        assertEquals(2, AncorErrorCode.INVALID_UTF8.value);
    }

    @Test
    void errorCode_internal_value_isThree() {
        assertEquals(3, AncorErrorCode.INTERNAL.value);
    }

    @Test
    void errorCode_ok_isOk() {
        assertTrue(AncorErrorCode.OK.isOk());
    }

    @Test
    void errorCode_nullPtr_notOk() {
        assertFalse(AncorErrorCode.NULL_PTR.isOk());
    }

    @Test
    void errorCode_invalidUtf8_notOk() {
        assertFalse(AncorErrorCode.INVALID_UTF8.isOk());
    }

    @Test
    void errorCode_internal_notOk() {
        assertFalse(AncorErrorCode.INTERNAL.isOk());
    }

    @Test
    void ancorException_code_roundTrip() {
        AncorException ex = new AncorException(AncorErrorCode.INVALID_UTF8, "bad utf8");
        assertEquals(2, ex.getErrorCode());
    }

    @Test
    void ancorException_inheritsRuntimeException() {
        assertTrue(RuntimeException.class.isAssignableFrom(AncorException.class));
    }

    @Test
    void ancorException_messageNotNull() {
        AncorException ex = new AncorException(AncorErrorCode.INTERNAL, "crash");
        assertNotNull(ex.getMessage());
    }

    @Test
    void ancorException_message_containsDescription() {
        AncorException ex = new AncorException(AncorErrorCode.NULL_PTR, "null pointer received");
        assertTrue(ex.getMessage().contains("null pointer received"));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
