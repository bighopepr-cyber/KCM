using System;
using System.IO;
using Kcm;

/// <summary>
/// KCM .NET SDK — Persistence Example.
/// Demonstrates: save, load, and verify database persistence.
/// </summary>
namespace Kcm.Examples
{
    public static class Persistence
    {
        public static void Run()
        {
            Console.WriteLine("=== KCM .NET SDK — Persistence Example ===\n");

            string path = Path.Combine(Path.GetTempPath(), "kcm_dotnet_example.kcm");

            try
            {
                // --- SAVE DATABASE ---
                Console.WriteLine("--- Save Database ---");
                using (var db = new KcmDatabase())
                {
                    db.Insert(new Fact(1, 0, 2, 0.95, evidence: 1, timestamp: 0, context: 1, version: 1, priority: 0, owner: 1));
                    db.Insert(new Fact(2, 1, 3, 0.90, evidence: 2, timestamp: 0, context: 1, version: 1, priority: 0, owner: 2));
                    db.Insert(new Fact(3, 2, 4, 0.85, evidence: 3, timestamp: 0, context: 2, version: 1, priority: 0, owner: 3));
                    Console.WriteLine($"  Facts before save: {db.FactCount()} total, {db.ActiveFactCount()} active");
                    db.Save(path);
                    Console.WriteLine($"  Saved to {path}");
                }

                // --- VERIFY FILE ---
                Console.WriteLine("\n--- Verify Database File ---");
                KcmDatabase.Verify(path);
                Console.WriteLine("  Verification passed");

                // --- LOAD INTO NEW DATABASE ---
                Console.WriteLine("\n--- Load Into New Database ---");
                using var db2 = new KcmDatabase();
                db2.Load(path);
                Console.WriteLine($"  Loaded: {db2.FactCount()} total, {db2.ActiveFactCount()} active");

                // --- VERIFY DATA INTEGRITY ---
                Console.WriteLine("\n--- Verify Data Integrity ---");
                using (var query = db2.Query("SELECT * FROM facts"))
                {
                    foreach (var f in query)
                    {
                        Console.WriteLine($"  Subject: {f.Subject}, Predicate: {f.Predicate}, Object: {f.Object}, Confidence: {f.Confidence:F2}");
                    }
                }

                // --- SAVE-LOAD ROUND TRIP ---
                Console.WriteLine("\n--- Save-Load Round Trip ---");
                db2.Insert(new Fact(10, 0, 20, 0.99));
                db2.Save(path);
                db2.Load(path);
                Console.WriteLine($"  Round-trip: {db2.FactCount()} total, {db2.ActiveFactCount()} active");
            }
            finally
            {
                if (File.Exists(path))
                    File.Delete(path);
            }

            Console.WriteLine("\n=== All persistence operations completed ===");
        }
    }
}
