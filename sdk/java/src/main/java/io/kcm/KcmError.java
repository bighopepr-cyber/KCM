package io.kcm;

/**
 * KCM error codes matching the C FFI enum.
 */
public enum KcmError {
    OK(0),
    NOT_FOUND(1),
    OUT_OF_MEMORY(2),
    INVALID_ARGUMENT(3),
    IO(4),
    CORRUPTED(5),
    CONFLICT(6),
    TRANSACTION_ABORTED(7);

    private final int code;

    KcmError(int code) { this.code = code; }

    public int getCode() { return code; }

    public static KcmError fromCode(int code) {
        for (KcmError e : values()) {
            if (e.code == code) return e;
        }
        throw new IllegalArgumentException("Unknown error code: " + code);
    }
}
