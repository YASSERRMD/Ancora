package io.ancora.ffi;

/** Error codes returned by all ancora-ffi functions. */
public enum AncorErrorCode {
    OK(0),
    NULL_PTR(1),
    INVALID_UTF8(2),
    INTERNAL(3);

    public final int value;

    AncorErrorCode(int value) {
        this.value = value;
    }

    public static AncorErrorCode from(int code) {
        for (AncorErrorCode e : values()) {
            if (e.value == code) return e;
        }
        return INTERNAL;
    }

    public boolean isOk() {
        return this == OK;
    }
}
