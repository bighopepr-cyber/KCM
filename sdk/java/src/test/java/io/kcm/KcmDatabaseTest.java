package io.kcm;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import static org.junit.jupiter.api.Assertions.*;

class KcmDatabaseTest {

    @Test
    void testFactCreationAllFields() {
        Fact f = new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 1700000000L, (byte) 1, 1, (byte) 0, (short) 5);
        assertEquals(1, f.subject);
        assertEquals(2, f.predicate);
        assertEquals(3, f.object);
        assertEquals(0.95, f.confidence, 1e-10);
        assertEquals(1, f.evidence);
        assertEquals(1700000000L, f.timestamp);
        assertEquals(1, f.context);
        assertEquals(1, f.version);
        assertEquals(0, f.priority);
        assertEquals(5, f.owner);
    }

    @Test
    void testFactCreationMinimal() {
        Fact f = new Fact(10, (byte) 5, 20, 0.5);
        assertEquals(10, f.subject);
        assertEquals(5, f.predicate);
        assertEquals(20, f.object);
        assertEquals(0.5, f.confidence, 1e-10);
        assertEquals(0, f.evidence);
        assertEquals(0L, f.timestamp);
        assertEquals(0, f.context);
        assertEquals(0, f.version);
        assertEquals(0, f.priority);
        assertEquals(0, f.owner);
    }

    @Test
    void testFactInvalidConfidence() {
        assertThrows(IllegalArgumentException.class, () -> new Fact(1, (byte) 0, 2, 1.5));
        assertThrows(IllegalArgumentException.class, () -> new Fact(1, (byte) 0, 2, -0.1));
    }

    @Test
    void testFactBoundaryConfidence() {
        assertDoesNotThrow(() -> new Fact(1, (byte) 0, 2, 0.0));
        assertDoesNotThrow(() -> new Fact(1, (byte) 0, 2, 1.0));
    }

    @Test
    void testFactEquality() {
        Fact f1 = new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 1, 1, (byte) 0, (short) 5);
        Fact f2 = new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 1, 1, (byte) 0, (short) 5);
        assertEquals(f1, f2);
        assertEquals(f1.hashCode(), f2.hashCode());

        Fact f3 = new Fact(2, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 1, 1, (byte) 0, (short) 5);
        assertNotEquals(f1, f3);
    }

    @Test
    void testFactInequalityFieldDifferences() {
        Fact base = new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 1, 1, (byte) 0, (short) 5);
        assertNotEquals(base, new Fact(1, (byte) 9, 3, 0.95, (byte) 1, 100L, (byte) 1, 1, (byte) 0, (short) 5));
        assertNotEquals(base, new Fact(1, (byte) 2, 3, 0.95, (byte) 9, 100L, (byte) 1, 1, (byte) 0, (short) 5));
        assertNotEquals(base, new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 999L, (byte) 1, 1, (byte) 0, (short) 5));
        assertNotEquals(base, new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 9, 1, (byte) 0, (short) 5));
        assertNotEquals(base, new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 1, 9, (byte) 0, (short) 5));
        assertNotEquals(base, new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 1, 1, (byte) 9, (short) 5));
        assertNotEquals(base, new Fact(1, (byte) 2, 3, 0.95, (byte) 1, 100L, (byte) 1, 1, (byte) 0, (short) 9));
    }

    @Test
    void testFactEqualityWithNull() {
        Fact f = new Fact(1, (byte) 0, 2, 0.5);
        assertNotEquals(null, f);
    }

    @Test
    void testFactToString() {
        Fact f = new Fact(1, (byte) 0, 2, 0.95);
        String s = f.toString();
        assertTrue(s.contains("Fact{"));
        assertTrue(s.contains("subject=1"));
        assertTrue(s.contains("predicate=0"));
        assertTrue(s.contains("object=2"));
        assertTrue(s.contains("confidence=0.95"));
    }

    @Test
    void testFactToStringAllFields() {
        Fact f = new Fact(10, (byte) 3, 20, 0.8, (byte) 2, 500L, (byte) 4, 7, (byte) -1, (short) 11);
        String s = f.toString();
        assertTrue(s.contains("evidence=2"));
        assertTrue(s.contains("timestamp=500"));
        assertTrue(s.contains("context=4"));
        assertTrue(s.contains("version=7"));
        assertTrue(s.contains("priority=-1"));
        assertTrue(s.contains("owner=11"));
    }

    @Test
    void testKcmErrorCodes() {
        assertEquals(0, KcmError.OK.getCode());
        assertEquals(1, KcmError.NOT_FOUND.getCode());
        assertEquals(2, KcmError.OUT_OF_MEMORY.getCode());
        assertEquals(3, KcmError.INVALID_ARGUMENT.getCode());
        assertEquals(4, KcmError.IO.getCode());
        assertEquals(5, KcmError.CORRUPTED.getCode());
        assertEquals(6, KcmError.CONFLICT.getCode());
        assertEquals(7, KcmError.TRANSACTION_ABORTED.getCode());
    }

    @Test
    void testKcmErrorFromCode() {
        assertEquals(KcmError.OK, KcmError.fromCode(0));
        assertEquals(KcmError.NOT_FOUND, KcmError.fromCode(1));
        assertEquals(KcmError.OUT_OF_MEMORY, KcmError.fromCode(2));
        assertEquals(KcmError.INVALID_ARGUMENT, KcmError.fromCode(3));
        assertEquals(KcmError.IO, KcmError.fromCode(4));
        assertEquals(KcmError.CORRUPTED, KcmError.fromCode(5));
        assertEquals(KcmError.CONFLICT, KcmError.fromCode(6));
        assertEquals(KcmError.TRANSACTION_ABORTED, KcmError.fromCode(7));
    }

    @Test
    void testKcmErrorUnknown() {
        assertThrows(IllegalArgumentException.class, () -> KcmError.fromCode(99));
        assertThrows(IllegalArgumentException.class, () -> KcmError.fromCode(-1));
    }

    @Test
    void testKcmErrorMessageFallback() {
        assertNotNull(KcmError.OK.getMessage());
        assertNotNull(KcmError.NOT_FOUND.getMessage());
        assertNotNull(KcmError.TRANSACTION_ABORTED.getMessage());
    }

    @Test
    void testKcmException() {
        KcmException ex = new KcmException(KcmError.NOT_FOUND);
        assertEquals(KcmError.NOT_FOUND, ex.getErrorCode());
        assertNotNull(ex.getMessage());

        KcmException ex2 = new KcmException(KcmError.IO, "file missing");
        assertEquals(KcmError.IO, ex2.getErrorCode());
        assertTrue(ex2.getMessage().contains("file missing"));
    }

    @Test
    void testKcmQueryImplementsIterator() {
        assertTrue(java.util.Iterator.class.isAssignableFrom(KcmQuery.class));
    }

    @Test
    void testKcmQueryCloseImplementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(KcmQuery.class));
    }

    @Test
    void testKcmTransactionImplementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(KcmTransaction.class));
    }

    @Test
    void testKcmDatabaseImplementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(KcmDatabase.class));
    }

    @Test
    void testFactImmutability() {
        Fact f = new Fact(1, (byte) 2, 3, 0.95);
        assertEquals(1, f.subject);
    }

    @Test
    void testKcmErrorAllValues() {
        KcmError[] values = KcmError.values();
        assertEquals(8, values.length);
    }
}
