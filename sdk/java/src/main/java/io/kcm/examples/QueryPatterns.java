package io.kcm.examples;

import io.kcm.*;

/**
 * KCM Java SDK — Query Patterns Example.
 *
 * Demonstrates: different KQL query patterns and filtering options.
 */
public class QueryPatterns {
    public static void main(String[] args) throws KcmException {
        System.out.println("=== KCM Java SDK — Query Patterns Example ===\n");

        try (KcmDatabase db = new KcmDatabase()) {

            // Insert test data
            db.insert(new Fact(1, (byte) 0, 2, 0.95, (byte) 1, 0L, (byte) 1, 1, (byte) 0, (short) 1));
            db.insert(new Fact(2, (byte) 1, 3, 0.90, (byte) 2, 0L, (byte) 1, 1, (byte) 0, (short) 2));
            db.insert(new Fact(3, (byte) 2, 4, 0.85, (byte) 3, 0L, (byte) 2, 1, (byte) 0, (short) 3));
            db.insert(new Fact(1, (byte) 3, 5, 0.80, (byte) 1, 0L, (byte) 2, 1, (byte) 0, (short) 1));
            db.insert(new Fact(4, (byte) 0, 6, 0.75, (byte) 2, 0L, (byte) 1, 1, (byte) 0, (short) 2));
            System.out.println("Inserted 5 facts\n");

            // --- SELECT ALL ---
            System.out.println("--- SELECT * FROM facts ---");
            try (KcmQuery query = db.query("SELECT * FROM facts")) {
                int count = 0;
                while (query.hasNext()) {
                    query.next();
                    count++;
                }
                System.out.printf("  Returned %d facts%n", count);
            }

            // --- FILTER BY SUBJECT ---
            System.out.println("\n--- Filter by Subject = 1 ---");
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

            // --- FILTER BY PREDICATE ---
            System.out.println("\n--- Filter by Predicate = 0 ---");
            try (KcmQuery query = db.query("SELECT * FROM facts")) {
                int count = 0;
                while (query.hasNext()) {
                    Fact f = query.next();
                    if (f.predicate == 0) {
                        System.out.printf("  Subject: %d, Predicate: %d, Object: %d%n",
                                f.subject, f.predicate, f.object);
                        count++;
                    }
                }
                System.out.printf("  Found %d facts with predicate=0%n", count);
            }

            // --- MULTI-CONDITION FILTER ---
            System.out.println("\n--- Filter: subject=1 AND predicate=3 ---");
            try (KcmQuery query = db.query("SELECT * FROM facts")) {
                int count = 0;
                while (query.hasNext()) {
                    Fact f = query.next();
                    if (f.subject == 1 && f.predicate == 3) {
                        System.out.printf("  Subject: %d, Predicate: %d, Object: %d%n",
                                f.subject, f.predicate, f.object);
                        count++;
                    }
                }
                System.out.printf("  Found %d facts matching multi-condition%n", count);
            }

            // --- ITERATOR PATTERN ---
            System.out.println("\n--- Iterator Pattern ---");
            try (KcmQuery query = db.query("SELECT * FROM facts")) {
                while (query.hasNext()) {
                    Fact f = query.next();
                    System.out.printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f%n",
                            f.subject, f.predicate, f.object, f.confidence);
                }
            }
        }

        System.out.println("\n=== All query patterns completed ===");
    }
}
