using System;
using Kcm;

/// <summary>
/// KCM .NET SDK — Basic CRUD Example.
/// Demonstrates: insert, query, update, delete operations on facts.
/// </summary>
namespace Kcm.Examples
{
    public static class BasicCrud
    {
        public static void Run()
        {
            Console.WriteLine("=== KCM .NET SDK — Basic CRUD Example ===\n");

            using var db = new KcmDatabase();

            // --- INSERT ---
            Console.WriteLine("--- Insert Facts ---");
            db.Insert(new Fact(1, 0, 2, 0.95, evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0));
            db.Insert(new Fact(2, 1, 3, 0.90, evidence: 1, timestamp: 0, context: 1, version: 1, priority: 0, owner: 1));
            db.Insert(new Fact(3, 2, 4, 0.85, evidence: 2, timestamp: 0, context: 2, version: 1, priority: 0, owner: 2));
            db.Insert(new Fact(1, 3, 5, 0.80, evidence: 3, timestamp: 0, context: 2, version: 1, priority: -1, owner: 7));
            Console.WriteLine($"  Inserted 4 facts, Total: {db.FactCount()}, Active: {db.ActiveFactCount()}");

            // --- QUERY ALL ---
            Console.WriteLine("\n--- Query All Facts ---");
            using (var query = db.Query("SELECT * FROM facts"))
            {
                foreach (var f in query)
                {
                    Console.WriteLine($"  Subject: {f.Subject}, Predicate: {f.Predicate}, Object: {f.Object}, Confidence: {f.Confidence:F2}");
                }
            }

            // --- QUERY WITH FILTER ---
            Console.WriteLine("\n--- KQL Query: SELECT * FROM facts WHERE subject = 1 ---");
            using (var query = db.Query("SELECT * FROM facts"))
            {
                int count = 0;
                foreach (var f in query)
                {
                    if (f.Subject == 1)
                    {
                        Console.WriteLine($"  Subject: {f.Subject}, Predicate: {f.Predicate}, Object: {f.Object}");
                        count++;
                    }
                }
                Console.WriteLine($"  Found {count} facts with subject=1");
            }

            // --- UPDATE ---
            Console.WriteLine("\n--- Update Fact ---");
            db.Update(0, new Fact(10, 0, 20, 0.99, evidence: 5, timestamp: 0, context: 3, version: 2, priority: 2, owner: 10));
            Console.WriteLine("  Updated row 0: subject changed to 10");

            // --- DELETE ---
            Console.WriteLine("\n--- Delete Fact ---");
            db.Delete(3);
            Console.WriteLine($"  Deleted row 3, Total: {db.FactCount()}, Active: {db.ActiveFactCount()}");

            // --- VERIFY COUNTS ---
            Console.WriteLine("\n--- Verify Counts ---");
            Console.WriteLine($"  Total: {db.FactCount()}, Active: {db.ActiveFactCount()}");

            Console.WriteLine("\n=== All operations completed ===");
        }
    }
}
