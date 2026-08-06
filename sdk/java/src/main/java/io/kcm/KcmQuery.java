package io.kcm;

import java.util.Iterator;
import java.util.NoSuchElementException;

public class KcmQuery implements AutoCloseable, Iterator<Fact> {
    private long nativeHandle;
    private Fact nextFact;
    private boolean exhausted;

    KcmQuery(long nativeHandle) {
        this.nativeHandle = nativeHandle;
        this.exhausted = false;
        prefetch();
    }

    @Override
    public boolean hasNext() {
        return nextFact != null;
    }

    @Override
    public Fact next() {
        if (nextFact == null) {
            throw new NoSuchElementException("No more query results");
        }
        Fact fact = nextFact;
        prefetch();
        return fact;
    }

    public long getNativeHandle() {
        return nativeHandle;
    }

    @Override
    public void close() {
        if (nativeHandle != 0) {
            KcmNative.nativeQueryFree(nativeHandle);
            nativeHandle = 0;
        }
    }

    private void prefetch() {
        if (exhausted || nativeHandle == 0) {
            nextFact = null;
            return;
        }
        nextFact = KcmNative.nativeQueryNext(nativeHandle);
        if (nextFact == null) {
            exhausted = true;
        }
    }
}
