package io.kcm;

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

    KcmError(int code) {
        this.code = code;
    }

    public int getCode() {
        return code;
    }

    public String getMessage() {
        try {
            return KcmNative.nativeErrorMessage(code);
        } catch (UnsatisfiedLinkError e) {
            return fallbackMessage();
        }
    }

    private String fallbackMessage() {
        switch (this) {
            case OK: return "Success";
            case NOT_FOUND: return "Not found";
            case OUT_OF_MEMORY: return "Out of memory";
            case INVALID_ARGUMENT: return "Invalid argument";
            case IO: return "I/O error";
            case CORRUPTED: return "Data corrupted";
            case CONFLICT: return "Conflict";
            case TRANSACTION_ABORTED: return "Transaction aborted";
            default: return "Unknown error";
        }
    }

    public static KcmError fromCode(int code) {
        for (KcmError e : values()) {
            if (e.code == code) return e;
        }
        throw new IllegalArgumentException("Unknown error code: " + code);
    }
}
