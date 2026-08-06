package io.kcm;

final class KcmNative {
    static {
        System.loadLibrary("kcm_jni");
    }

    private KcmNative() {}

    static native int nativeDatabaseNew(long[] dbOut);
    static native void nativeDatabaseFree(long db);
    static native int nativeDatabaseInsert(long db, int subject, byte predicate, int object,
                                           double confidence, byte evidence, long timestamp,
                                           byte context, int version, byte priority, short owner);
    static native int nativeDatabaseUpdate(long db, long rowId, int subject, byte predicate, int object,
                                           double confidence, byte evidence, long timestamp,
                                           byte context, int version, byte priority, short owner);
    static native int nativeDatabaseDelete(long db, long rowId);
    static native long nativeDatabaseFactCount(long db);
    static native long nativeDatabaseActiveCount(long db);
    static native long nativeDatabaseQuery(long db, String query);
    static native Fact nativeQueryNext(long query);
    static native void nativeQueryFree(long query);
    static native long nativeDatabaseBeginTransaction(long db);
    static native int nativeTransactionCommit(long txn);
    static native int nativeTransactionRollback(long txn);
    static native void nativeTransactionFree(long txn);
    static native int nativeDatabaseSave(long db, String path);
    static native int nativeDatabaseLoad(long db, String path);
    static native int nativeDatabaseVerify(String path);
    static native String nativeErrorMessage(int err);
}
