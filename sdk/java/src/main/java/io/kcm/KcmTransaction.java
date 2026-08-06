package io.kcm;

public class KcmTransaction implements AutoCloseable {
    private long nativeHandle;
    private boolean committed;
    private boolean rolledBack;

    KcmTransaction(long nativeHandle) {
        this.nativeHandle = nativeHandle;
        this.committed = false;
        this.rolledBack = false;
    }

    public void commit() throws KcmException {
        if (nativeHandle == 0) {
            throw new IllegalStateException("Transaction is closed");
        }
        int err = KcmNative.nativeTransactionCommit(nativeHandle);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
        committed = true;
    }

    public void rollback() throws KcmException {
        if (nativeHandle == 0) {
            throw new IllegalStateException("Transaction is closed");
        }
        int err = KcmNative.nativeTransactionRollback(nativeHandle);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
        rolledBack = true;
    }

    public boolean isCommitted() {
        return committed;
    }

    public boolean isRolledBack() {
        return rolledBack;
    }

    public long getNativeHandle() {
        return nativeHandle;
    }

    @Override
    public void close() {
        if (nativeHandle != 0) {
            if (!committed && !rolledBack) {
                try {
                    KcmNative.nativeTransactionRollback(nativeHandle);
                } catch (UnsatisfiedLinkError ignored) {
                }
            }
            KcmNative.nativeTransactionFree(nativeHandle);
            nativeHandle = 0;
        }
    }
}
