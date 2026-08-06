using System;
using Kcm;

/// <summary>
/// KCM .NET SDK — Transaction Example.
/// Demonstrates: begin, commit, and rollback scenarios with transactions.
/// </summary>
namespace Kcm.Examples
{
    public static class Transactions
    {
        public static void Run()
        {
            Console.WriteLine("=== KCM .NET SDK — Transaction Example ===\n");

            using var db = new KcmDatabase();

            // Insert baseline facts
            db.Insert(new Fact(1, 0, 2, 0.95));
            db.Insert(new Fact(2, 1, 3, 0.90));
            Console.WriteLine($"Initial: {db.ActiveFactCount()} active facts\n");

            // --- COMMITTED TRANSACTION ---
            Console.WriteLine("--- Committed Transaction ---");
            using (var txn = db.BeginTransaction())
            {
                db.Insert(new Fact(3, 2, 4, 0.85, evidence: 2, timestamp: 0, context: 2, version: 1, priority: 0, owner: 2));
                Console.WriteLine("  Inserted fact in transaction");
                txn.Commit();
                Console.WriteLine("  Committed transaction");
            }
            Console.WriteLine($"  After commit: {db.ActiveFactCount()} active facts");

            // --- ROLLED BACK TRANSACTION ---
            Console.WriteLine("\n--- Rolled Back Transaction ---");
            using (var txn = db.BeginTransaction())
            {
                db.Insert(new Fact(4, 3, 5, 0.80, evidence: 3, timestamp: 0, context: 2, version: 1, priority: 0, owner: 3));
                Console.WriteLine("  Inserted fact in transaction");
                txn.Rollback();
                Console.WriteLine("  Rolled back transaction");
            }
            Console.WriteLine($"  After rollback: {db.ActiveFactCount()} active facts");

            // --- AUTO-ROLLBACK ON EXCEPTION ---
            Console.WriteLine("\n--- Auto-Rollback on Exception ---");
            ulong countBefore = db.ActiveFactCount();
            try
            {
                using var txn = db.BeginTransaction();
                db.Insert(new Fact(5, 4, 6, 0.70));
                throw new InvalidOperationException("simulated error");
            }
            catch (InvalidOperationException e)
            {
                Console.WriteLine($"  Caught simulated error: {e.Message}");
            }
            Console.WriteLine($"  After exception: {db.ActiveFactCount()} active facts");
            Console.WriteLine($"  Transaction auto-rolled back: {db.ActiveFactCount() == countBefore}");

            // --- MULTIPLE OPERATIONS ---
            Console.WriteLine("\n--- Multiple Operations in Transaction ---");
            using (var txn = db.BeginTransaction())
            {
                db.Insert(new Fact(10, 0, 20, 0.99));
                db.Insert(new Fact(30, 1, 40, 0.88));
                db.Insert(new Fact(50, 2, 60, 0.77));
                Console.WriteLine("  3 pending operations");
                txn.Commit();
            }
            Console.WriteLine($"  After commit: {db.ActiveFactCount()} active facts");

            Console.WriteLine("\n=== All transaction operations completed ===");
        }
    }
}
