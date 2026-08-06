package io.kcm;

public class KcmException extends Exception {
    private final KcmError errorCode;

    public KcmException(KcmError errorCode) {
        super(errorCode.getMessage());
        this.errorCode = errorCode;
    }

    public KcmException(KcmError errorCode, String detail) {
        super(errorCode.getMessage() + ": " + detail);
        this.errorCode = errorCode;
    }

    public KcmError getErrorCode() {
        return errorCode;
    }
}
