package io.kcm;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class KcmDatabaseTest {

    @Test
    void testFactCreation() {
        Fact f = new Fact(1, (byte) 0, 2, 0.95);
        assertEquals(1, f.subject);
        assertEquals(0, f.predicate);
        assertEquals(2, f.object);
        assertEquals(0.95, f.confidence, 1e-10);
    }

    @Test
    void testFactInvalidConfidence() {
        assertThrows(IllegalArgumentException.class, () -> new Fact(1, (byte) 0, 2, 1.5));
        assertThrows(IllegalArgumentException.class, () -> new Fact(1, (byte) 0, 2, -0.1));
    }

    @Test
    void testFactToString() {
        Fact f = new Fact(1, (byte) 0, 2, 0.95);
        assertTrue(f.toString().contains("Fact"));
        assertTrue(f.toString().contains("subject=1"));
    }

    @Test
    void testFactEquality() {
        Fact f1 = new Fact(1, (byte) 0, 2, 0.95);
        Fact f2 = new Fact(1, (byte) 0, 2, 0.95);
        assertEquals(f1, f2);

        Fact f3 = new Fact(2, (byte) 0, 2, 0.95);
        assertNotEquals(f1, f3);
    }

    @Test
    void testKcmErrorCodes() {
        assertEquals(0, KcmError.OK.getCode());
        assertEquals(1, KcmError.NOT_FOUND.getCode());
        assertEquals(3, KcmError.INVALID_ARGUMENT.getCode());
        assertEquals(KcmError.OK, KcmError.fromCode(0));
        assertEquals(KcmError.NOT_FOUND, KcmError.fromCode(1));
    }

    @Test
    void testKcmErrorUnknown() {
        assertThrows(IllegalArgumentException.class, () -> KcmError.fromCode(99));
    }
}
