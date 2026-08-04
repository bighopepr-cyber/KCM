package io.kcm;

/**
 * KCM Knowledge Columnar Model - Java SDK
 *
 * JNI-based bindings to the KCM C FFI.
 *
 * Example usage:
 * <pre>
 * KcmDatabase db = new KcmDatabase();
 * db.insert(new Fact(1, (byte) 0, 2, 0.95));
 * List&lt;Fact&gt; facts = db.queryAll();
 * for (Fact f : facts) {
 *     System.out.println(f);
 * }
 * db.close();
 * </pre>
 */
public class KcmDatabase {

    static {
        System.loadLibrary("kcm");
    }

    private long nativeHandle;

    public KcmDatabase() {
        this.nativeHandle = nativeNew();
    }

    public void close() {
        if (nativeHandle != 0) {
            nativeFree(nativeHandle);
            nativeHandle = 0;
        }
    }

    public void insert(Fact fact) {
        checkHandle();
        nativeInsert(nativeHandle, fact.subject, fact.predicate, fact.object, fact.confidence);
    }

    public void delete(long rowId) {
        checkHandle();
        nativeDelete(nativeHandle, rowId);
    }

    public long factCount() {
        checkHandle();
        return nativeFactCount(nativeHandle);
    }

    public long activeFactCount() {
        checkHandle();
        return nativeActiveCount(nativeHandle);
    }

    @Override
    protected void finalize() throws Throwable {
        close();
        super.finalize();
    }

    private void checkHandle() {
        if (nativeHandle == 0) {
            throw new IllegalStateException("Database is closed");
        }
    }

    // Native methods
    private static native long nativeNew();
    private static native void nativeFree(long handle);
    private static native void nativeInsert(long handle, int subject, int predicate, int object, double confidence);
    private static native void nativeDelete(long handle, long rowId);
    private static native long nativeFactCount(long handle);
    private static native long nativeActiveCount(long handle);
}
