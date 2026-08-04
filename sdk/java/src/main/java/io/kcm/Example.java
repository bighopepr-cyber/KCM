package io.kcm;

/**
 * KCM Java SDK Example
 */
public class Example {
    public static void main(String[] args) {
        System.out.println("=== KCM Java SDK Example ===\n");

        // Create facts
        Fact f1 = new Fact(1, (byte) 0, 2, 0.95);
        Fact f2 = new Fact(2, (byte) 1, 3, 0.90);
        Fact f3 = new Fact(3, (byte) 2, 4, 0.85);

        System.out.println("Fact 1: " + f1);
        System.out.println("Fact 2: " + f2);
        System.out.println("Fact 3: " + f3);

        // Test equality
        Fact f1copy = new Fact(1, (byte) 0, 2, 0.95);
        System.out.println("Fact 1 == copy: " + f1.equals(f1copy));

        // Test invalid confidence
        try {
            new Fact(1, (byte) 0, 2, 1.5);
            System.out.println("FAIL: Should have thrown");
        } catch (IllegalArgumentException e) {
            System.out.println("Invalid confidence rejected: " + e.getMessage());
        }

        // Test error codes
        System.out.println("OK code: " + KcmError.OK.getCode());
        System.out.println("NOT_FOUND code: " + KcmError.NOT_FOUND.getCode());
        System.out.println("OK fromCode: " + KcmError.fromCode(0));

        System.out.println("\nAll Java SDK examples completed!");
    }
}
