using System;
using Kcm;

/// <summary>
/// KCM .NET SDK — Error Handling Example.
/// Demonstrates: proper error handling patterns with KcmException and KcmError.
/// </summary>
namespace Kcm.Examples
{
    public static class ErrorHandling
    {
        public static void Run()
        {
            Console.WriteLine("=== KCM .NET SDK — Error Handling Example ===\n");

            // --- INVALID CONFIDENCE ---
            Console.WriteLine("--- Invalid Confidence (out of range) ---");
            try
            {
                var bad = new Fact(1, 0, 2, 1.5);
                Console.WriteLine("  FAIL: Should have thrown");
            }
            catch (ArgumentException e)
            {
                Console.WriteLine($"  Caught: {e.Message}");
            }

            // --- DATABASE DISPOSED ---
            Console.WriteLine("\n--- Database Disposed ---");
            try
            {
                var db2 = new KcmDatabase();
                db2.Dispose();
                db2.Insert(new Fact(1, 0, 2, 0.5));
                Console.WriteLine("  FAIL: Should have thrown");
            }
            catch (ObjectDisposedException e)
            {
                Console.WriteLine($"  Caught: {e.Message}");
            }

            // --- NOT FOUND (update non-existent row) ---
            Console.WriteLine("\n--- Not Found (update non-existent row) ---");
            try
            {
                using var db = new KcmDatabase();
                db.Update(99999, new Fact(1, 0, 2, 0.5));
                Console.WriteLine("  FAIL: Should have thrown");
            }
            catch (KcmException e)
            {
                Console.WriteLine($"  Caught KcmException: code={e.ErrorCode}");
                Console.WriteLine($"  Message: {e.Message}");
            }

            // --- FILE NOT FOUND (load) ---
            Console.WriteLine("\n--- File Not Found (load) ---");
            try
            {
                using var db = new KcmDatabase();
                db.Load("/nonexistent/path/db.kcm");
                Console.WriteLine("  FAIL: Should have thrown");
            }
            catch (KcmException e)
            {
                Console.WriteLine($"  Caught KcmException: code={e.ErrorCode}");
                Console.WriteLine($"  Message: {e.Message}");
            }

            // --- ALL ERROR CODES ---
            Console.WriteLine("\n--- All Error Codes ---");
            foreach (KcmError code in Enum.GetValues<KcmError>())
            {
                var ex = new KcmException(code);
                Console.WriteLine($"  {code} ({(int)code}): {ex.Message}");
            }

            // --- TRY-CATCH PATTERN ---
            Console.WriteLine("\n--- Try-Catch Pattern ---");
            try
            {
                using var db = new KcmDatabase();
                db.Insert(new Fact(1, 0, 2, 0.95));
                db.Insert(new Fact(2, 1, 3, 0.90));
                using var query = db.Query("SELECT * FROM facts");
                int count = 0;
                foreach (var _ in query) count++;
                Console.WriteLine($"  Query returned {count} results");
            }
            catch (KcmException e)
            {
                Console.WriteLine($"  Database error: {e.ErrorCode}: {e.Message}");
            }

            Console.WriteLine("\n=== All error handling patterns completed ===");
        }
    }
}
