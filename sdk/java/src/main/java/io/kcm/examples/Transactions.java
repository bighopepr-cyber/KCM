package io.kcm.examples;

import io.kcm.*;

/**
 * KCM Java SDK — Transaction Example.
 *
 * Demonstrates: begin, commit, and rollback scenarios with transactions.
 */
public class Transactions {
    public static void main(String[] args) throws KcmException {
        System.out.println("=== KCM Java SDK — Transaction Example ===\n");

        try (KcmDatabase db = new KcmDatabase()) {

            // Insert baseline facts
            db.insert(new Fact(1, (byte) 0, 2, 0.95));
            db.insert(new Fact(2, (byte) 1, 3, 0.90));
            System.out.printf("Initial: %d active facts%n%n", db.activeFactCount());

            // --- COMMITTED TRANSACTION ---
            System.out.println("--- Committed Transaction ---");
            try (KcmTransaction txn = db.beginTransaction()) {
                db.insert(new Fact(3, (byte) 2, 4, 0.85,
                        (byte) 2, 0L, (byte) 2, 1, (byte) 0, (short) 2));
                System.out.println("  Inserted fact in transaction");
                txn.commit();
                System.out.println("  Committed transaction");
            }
            System.out.printf("  After commit: %d active facts%n", db.activeFactCount());
            assert db.activeFactCount() == 3;

            // --- ROLLED BACK TRANSACTION ---
            System.out.println("\n--- Rolled Back Transaction ---");
            try (KcmTransaction txn = db.beginTransaction()) {
                db.insert(new Fact(4, (byte) 3, 5, 0.80,
                        (byte) 3, 0L, (byte) 2, 1, (byte) 0, (short) 3));
                System.out.println("  Inserted fact in transaction");
                txn.rollback();
                System.out.println("  Rolled back transaction");
            }
            System.out.printf("  After rollback: %d active facts%n", db.activeFactCount());
            assert db.activeFactCount() == 3;

            // --- AUTO-ROLLBACK ON EXCEPTION ---
            System.out.println("\n--- Auto-Rollback on Exception ---");
            long countBefore = db.activeFactCount();
            try (KcmTransaction txn = db.beginTransaction()) {
                db.insert(new Fact(5, (byte) 4, 6, 0.70));
                throw new RuntimeException("simulated error");
            } catch (RuntimeException e) {
                System.out.println("  Caught simulated error: " + e.getMessage());
            }
            System.out.printf("  After exception: %d active facts%n", db.activeFactCount());
            System.out.printf("  Transaction auto-rolled back: %b%n",
                    db.activeFactCount() == countBefore);

            // --- MULTIPLE OPERATIONS ---
            System.out.println("\n--- Multiple Operations in Transaction ---");
            try (KcmTransaction txn = db.beginTransaction()) {
                db.insert(new Fact(10, (byte) 0, 20, 0.99));
                db.insert(new Fact(30, (byte) 1, 40, 0.88));
                db.insert(new Fact(50, (byte) 2, 60, 0.77));
                System.out.println("  3 pending operations");
                txn.commit();
            }
            System.out.printf("  After commit: %d active facts%n", db.activeFactCount());
        }

        System.out.println("\n=== All transaction operations completed ===");
    }
}
