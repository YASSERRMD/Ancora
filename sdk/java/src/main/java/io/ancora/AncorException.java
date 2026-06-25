package io.ancora;

import io.ancora.ffi.AncorErrorCode;

/** Exception raised when an ancora-ffi function returns a non-OK error code. */
public final class AncorException extends Exception {

    private final int errorCode;

    public AncorException(int errorCode, String message) {
        super(message + " (ErrorCode=" + errorCode + ":" + AncorErrorCode.from(errorCode) + ")");
        this.errorCode = errorCode;
    }

    public AncorException(AncorErrorCode code, String message) {
        this(code.value, message);
    }

    public int getErrorCode() {
        return errorCode;
    }
}
