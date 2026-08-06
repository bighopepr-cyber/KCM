package io.kcm.examples;

import io.kcm.*;

/**
 * KCM Java SDK — Error Handling Example.
 *
 * Demonstrates: proper error handling patterns with KcmException and KcmError.
 */
public class ErrorHandling {
    public static void main(String[] args) {
        System.out.println("=== KCM Java SDK — Error Handling Example ===\n");

        // --- INVALID CONFIDENCE ---
        System.out.println("--- Invalid Confidence (out of range) ---");
        try {
            new Fact(1, (byte) 0, 2, 1.5);
            System.out.println("  FAIL: Should have thrown");
        } catch (IllegalArgumentException e) {
            System.out.println("  Caught: " + e.getMessage());
        }

        // --- DATABASE CLOSED ---
        System.out.println("\n--- Database Closed ---");
        try {
            KcmDatabase db2 = new KcmDatabase();
            db2.close();
            db2.insert(new Fact(1, (byte) 0, 2, 0.5));
            System.out.println("  FAIL: Should have thrown");
        } catch (IllegalStateException e) {
            System.out.println("  Caught: " + e.getMessage());
        } catch (KcmException e) {
            System.out.println("  Caught KcmException: " + e.getMessage());
        }

        // --- NOT FOUND (update non-existent row) ---
        System.out.println("\n--- Not Found (update non-existent row) ---");
        try (KcmDatabase db = new KcmDatabase()) {
            db.update(99999, new Fact(1, (byte) 0, 2, 0.5));
            System.out.println("  FAIL: Should have thrown");
        } catch (KcmException e) {
            System.out.println("  Caught KcmException: code=" + e.getErrorCode());
            System.out.println("  Message: " + e.getMessage());
        }

        // --- NOT FOUND (delete non-existent row) ---
        System.out.println("\n--- Not Found (delete non-existent row) ---");
        try (KcmDatabase db = new KcmDatabase()) {
            db.delete(99999);
            System.out.println("  Delete succeeded (no exception)");
        } catch (KcmException e) {
            System.out.println("  Caught KcmException: " + e.getMessage());
        }

        // --- FILE NOT FOUND (load) ---
        System.out.println("\n--- File Not Found (load) ---");
        try (KcmDatabase db = new KcmDatabase()) {
            db.load("/nonexistent/path/db.kcm");
            System.out.println("  FAIL: Should have thrown");
        } catch (KcmException e) {
            System.out.println("  Caught KcmException: code=" + e.getErrorCode());
            System.out.println("  Message: " + e.getMessage());
        }

        // --- ALL ERROR CODES ---
        System.out.println("\n--- All Error Codes ---");
        for (KcmError code : KcmError.values()) {
            System.out.printf("  %s (%d): %s%n", code.name(), code.getCode(), code.getMessage());
        }

        // --- TRY-CATCH PATTERN ---
        System.out.println("\n--- Try-Catch Pattern ---");
        try (KcmDatabase db = new KcmDatabase()) {
            db.insert(new Fact(1, (byte) 0, 2, 0.95));
            db.insert(new Fact(2, (byte) 1, 3, 0.90));
            try (KcmQuery query = db.query("SELECT * FROM facts")) {
                int count = 0;
                while (query.hasNext()) {
                    query.next();
                    count++;
                }
                System.out.printf("  Query returned %d results%n", count);
            }
        } catch (KcmException e) {
            System.out.println("  Database error: " + e.getErrorCode() + ": " + e.getMessage());
        }

        System.out.println("\n=== All error handling patterns completed ===");
    }
}
