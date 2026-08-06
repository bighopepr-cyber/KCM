package io.kcm.examples;

import io.kcm.*;

/**
 * KCM Java SDK — Basic CRUD Example.
 *
 * Demonstrates: insert, query, update, delete operations on facts.
 */
public class BasicCrud {
    public static void main(String[] args) throws KcmException {
        System.out.println("=== KCM Java SDK — Basic CRUD Example ===\n");

        try (KcmDatabase db = new KcmDatabase()) {

            // --- INSERT ---
            System.out.println("--- Insert Facts ---");
            db.insert(new Fact(1, (byte) 0, 2, 0.95,
                    (byte) 0, 0L, (byte) 0, 1, (byte) 0, (short) 0));
            db.insert(new Fact(2, (byte) 1, 3, 0.90,
                    (byte) 1, 0L, (byte) 1, 1, (byte) 0, (short) 1));
            db.insert(new Fact(3, (byte) 2, 4, 0.85,
                    (byte) 2, 0L, (byte) 2, 1, (byte) 0, (short) 2));
            db.insert(new Fact(1, (byte) 3, 5, 0.80,
                    (byte) 3, 0L, (byte) 2, 1, (byte) -1, (short) 7));
            System.out.printf("  Inserted 4 facts, Total: %d, Active: %d%n",
                    db.factCount(), db.activeFactCount());

            // --- QUERY ALL ---
            System.out.println("\n--- Query All Facts ---");
            try (KcmQuery query = db.query("SELECT * FROM facts")) {
                while (query.hasNext()) {
                    Fact f = query.next();
                    System.out.printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f%n",
                            f.subject, f.predicate, f.object, f.confidence);
                }
            }

            // --- QUERY WITH FILTER ---
            System.out.println("\n--- KQL Query: SELECT * FROM facts WHERE subject = 1 ---");
            try (KcmQuery query = db.query("SELECT * FROM facts")) {
                int count = 0;
                while (query.hasNext()) {
                    Fact f = query.next();
                    if (f.subject == 1) {
                        System.out.printf("  Subject: %d, Predicate: %d, Object: %d%n",
                                f.subject, f.predicate, f.object);
                        count++;
                    }
                }
                System.out.printf("  Found %d facts with subject=1%n", count);
            }

            // --- UPDATE ---
            System.out.println("\n--- Update Fact ---");
            db.update(0, new Fact(10, (byte) 0, 20, 0.99,
                    (byte) 5, 0L, (byte) 3, 2, (byte) 2, (short) 10));
            System.out.println("  Updated row 0: subject changed to 10");

            // --- DELETE ---
            System.out.println("\n--- Delete Fact ---");
            db.delete(3);
            System.out.printf("  Deleted row 3, Total: %d, Active: %d%n",
                    db.factCount(), db.activeFactCount());

            // --- VERIFY COUNTS ---
            System.out.println("\n--- Verify Counts ---");
            assert db.factCount() == 4 : "Expected 4 total, got " + db.factCount();
            assert db.activeFactCount() == 3 : "Expected 3 active, got " + db.activeFactCount();
            System.out.println("  Counts verified: 4 total, 3 active");
        }

        System.out.println("\n=== All operations completed ===");
    }
}
