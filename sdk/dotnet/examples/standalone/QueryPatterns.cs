using System;
using Kcm;

/// <summary>
/// KCM .NET SDK — Query Patterns Example.
/// Demonstrates: different KQL query patterns and filtering options.
/// </summary>
namespace Kcm.Examples
{
    public static class QueryPatterns
    {
        public static void Run()
        {
            Console.WriteLine("=== KCM .NET SDK — Query Patterns Example ===\n");

            using var db = new KcmDatabase();

            // Insert test data
            db.Insert(new Fact(1, 0, 2, 0.95, evidence: 1, timestamp: 0, context: 1, version: 1, priority: 0, owner: 1));
            db.Insert(new Fact(2, 1, 3, 0.90, evidence: 2, timestamp: 0, context: 1, version: 1, priority: 0, owner: 2));
            db.Insert(new Fact(3, 2, 4, 0.85, evidence: 3, timestamp: 0, context: 2, version: 1, priority: 0, owner: 3));
            db.Insert(new Fact(1, 3, 5, 0.80, evidence: 1, timestamp: 0, context: 2, version: 1, priority: 0, owner: 1));
            db.Insert(new Fact(4, 0, 6, 0.75, evidence: 2, timestamp: 0, context: 1, version: 1, priority: 0, owner: 2));
            Console.WriteLine("Inserted 5 facts\n");

            // --- SELECT ALL ---
            Console.WriteLine("--- SELECT * FROM facts ---");
            using (var query = db.Query("SELECT * FROM facts"))
            {
                int count = 0;
                foreach (var _ in query) count++;
                Console.WriteLine($"  Returned {count} facts");
            }

            // --- FILTER BY SUBJECT ---
            Console.WriteLine("\n--- Filter by Subject = 1 ---");
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

            // --- FILTER BY PREDICATE ---
            Console.WriteLine("\n--- Filter by Predicate = 0 ---");
            using (var query = db.Query("SELECT * FROM facts"))
            {
                int count = 0;
                foreach (var f in query)
                {
                    if (f.Predicate == 0)
                    {
                        Console.WriteLine($"  Subject: {f.Subject}, Predicate: {f.Predicate}, Object: {f.Object}");
                        count++;
                    }
                }
                Console.WriteLine($"  Found {count} facts with predicate=0");
            }

            // --- MULTI-CONDITION FILTER ---
            Console.WriteLine("\n--- Filter: subject=1 AND predicate=3 ---");
            using (var query = db.Query("SELECT * FROM facts"))
            {
                int count = 0;
                foreach (var f in query)
                {
                    if (f.Subject == 1 && f.Predicate == 3)
                    {
                        Console.WriteLine($"  Subject: {f.Subject}, Predicate: {f.Predicate}, Object: {f.Object}");
                        count++;
                    }
                }
                Console.WriteLine($"  Found {count} facts matching multi-condition");
            }

            // --- ITERATOR PATTERN ---
            Console.WriteLine("\n--- Iterator Pattern ---");
            using (var query = db.Query("SELECT * FROM facts"))
            {
                foreach (var f in query)
                {
                    Console.WriteLine($"  Subject: {f.Subject}, Predicate: {f.Predicate}, Object: {f.Object}, Confidence: {f.Confidence:F2}");
                }
            }

            Console.WriteLine("\n=== All query patterns completed ===");
        }
    }
}
