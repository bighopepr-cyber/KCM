package io.kcm;

public class KcmDatabase implements AutoCloseable {
    private long nativeHandle;

    public KcmDatabase() throws KcmException {
        long[] handleOut = new long[1];
        int err = KcmNative.nativeDatabaseNew(handleOut);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
        this.nativeHandle = handleOut[0];
    }

    KcmDatabase(long nativeHandle) {
        this.nativeHandle = nativeHandle;
    }

    public void insert(Fact fact) throws KcmException {
        checkOpen();
        int err = KcmNative.nativeDatabaseInsert(nativeHandle,
                fact.subject, fact.predicate, fact.object, fact.confidence,
                fact.evidence, fact.timestamp, fact.context, fact.version,
                fact.priority, fact.owner);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
    }

    public void update(long rowId, Fact fact) throws KcmException {
        checkOpen();
        int err = KcmNative.nativeDatabaseUpdate(nativeHandle, rowId,
                fact.subject, fact.predicate, fact.object, fact.confidence,
                fact.evidence, fact.timestamp, fact.context, fact.version,
                fact.priority, fact.owner);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
    }

    public void delete(long rowId) throws KcmException {
        checkOpen();
        int err = KcmNative.nativeDatabaseDelete(nativeHandle, rowId);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
    }

    public long factCount() {
        checkOpen();
        return KcmNative.nativeDatabaseFactCount(nativeHandle);
    }

    public long activeFactCount() {
        checkOpen();
        return KcmNative.nativeDatabaseActiveCount(nativeHandle);
    }

    public KcmQuery query(String kql) throws KcmException {
        checkOpen();
        long queryHandle = KcmNative.nativeDatabaseQuery(nativeHandle, kql);
        if (queryHandle == 0) {
            throw new KcmException(KcmError.IO, "Query returned null handle");
        }
        return new KcmQuery(queryHandle);
    }

    public KcmTransaction beginTransaction() throws KcmException {
        checkOpen();
        long txnHandle = KcmNative.nativeDatabaseBeginTransaction(nativeHandle);
        if (txnHandle == 0) {
            throw new KcmException(KcmError.IO, "Transaction returned null handle");
        }
        return new KcmTransaction(txnHandle);
    }

    public void save(String path) throws KcmException {
        checkOpen();
        int err = KcmNative.nativeDatabaseSave(nativeHandle, path);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
    }

    public void load(String path) throws KcmException {
        checkOpen();
        int err = KcmNative.nativeDatabaseLoad(nativeHandle, path);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
    }

    public void verify(String path) throws KcmException {
        int err = KcmNative.nativeDatabaseVerify(path);
        if (err != KcmError.OK.getCode()) {
            throw new KcmException(KcmError.fromCode(err));
        }
    }

    public boolean isOpen() {
        return nativeHandle != 0;
    }

    @Override
    public void close() {
        if (nativeHandle != 0) {
            KcmNative.nativeDatabaseFree(nativeHandle);
            nativeHandle = 0;
        }
    }

    long getNativeHandle() {
        return nativeHandle;
    }

    private void checkOpen() {
        if (nativeHandle == 0) {
            throw new IllegalStateException("Database is closed");
        }
    }
}
