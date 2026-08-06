package io.kcm.examples;

import io.kcm.*;
import java.io.File;

/**
 * KCM Java SDK — Persistence Example.
 *
 * Demonstrates: save, load, and verify database persistence.
 */
public class Persistence {
    public static void main(String[] args) throws KcmException {
        System.out.println("=== KCM Java SDK — Persistence Example ===\n");

        String path = System.getProperty("java.io.tmpdir") + File.separator + "kcm_java_example.kcm";

        try {
            // --- SAVE DATABASE ---
            System.out.println("--- Save Database ---");
            try (KcmDatabase db = new KcmDatabase()) {
                db.insert(new Fact(1, (byte) 0, 2, 0.95,
                        (byte) 1, 0L, (byte) 1, 1, (byte) 0, (short) 1));
                db.insert(new Fact(2, (byte) 1, 3, 0.90,
                        (byte) 2, 0L, (byte) 1, 1, (byte) 0, (short) 2));
                db.insert(new Fact(3, (byte) 2, 4, 0.85,
                        (byte) 3, 0L, (byte) 2, 1, (byte) 0, (short) 3));
                System.out.printf("  Facts before save: %d total, %d active%n",
                        db.factCount(), db.activeFactCount());
                db.save(path);
                System.out.printf("  Saved to %s%n", path);
            }

            // --- VERIFY FILE ---
            System.out.println("\n--- Verify Database File ---");
            try (KcmDatabase db = new KcmDatabase()) {
                db.verify(path);
                System.out.println("  Verification passed");
            }

            // --- LOAD INTO NEW DATABASE ---
            System.out.println("\n--- Load Into New Database ---");
            try (KcmDatabase db2 = new KcmDatabase()) {
                db2.load(path);
                System.out.printf("  Loaded: %d total, %d active%n",
                        db2.factCount(), db2.activeFactCount());
                assert db2.factCount() == 3;
                assert db2.activeFactCount() == 3;

                // --- VERIFY DATA INTEGRITY ---
                System.out.println("\n--- Verify Data Integrity ---");
                try (KcmQuery query = db2.query("SELECT * FROM facts")) {
                    while (query.hasNext()) {
                        Fact f = query.next();
                        System.out.printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f%n",
                                f.subject, f.predicate, f.object, f.confidence);
                    }
                }

                // --- SAVE-LOAD ROUND TRIP ---
                System.out.println("\n--- Save-Load Round Trip ---");
                db2.insert(new Fact(10, (byte) 0, 20, 0.99));
                db2.save(path);
            }

            try (KcmDatabase db3 = new KcmDatabase()) {
                db3.load(path);
                System.out.printf("  Round-trip: %d total, %d active%n",
                        db3.factCount(), db3.activeFactCount());
                assert db3.factCount() == 4;
                assert db3.activeFactCount() == 4;
            }
        } finally {
            new File(path).delete();
        }

        System.out.println("\n=== All persistence operations completed ===");
    }
}
