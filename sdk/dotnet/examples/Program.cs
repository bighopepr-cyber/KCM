using System;
using Kcm;

Console.WriteLine("=== KCM .NET SDK Example ===\n");

using var db = new KcmDatabase();
Console.WriteLine($"Database created. Fact count: {db.FactCount()}");

var facts = new Fact[]
{
    new(1, 0, 2, 0.95, evidence: 1, timestamp: 1700000000000000000, context: 1, version: 1, priority: 0, owner: 1),
    new(2, 1, 3, 0.90, evidence: 2, timestamp: 1700000001000000000, context: 1, version: 1, priority: 1, owner: 2),
    new(3, 2, 4, 0.85, evidence: 3, timestamp: 1700000002000000000, context: 2, version: 2, priority: -1, owner: 3),
};

Console.WriteLine("\n--- Insert Facts ---");
foreach (var fact in facts)
{
    db.Insert(fact);
    Console.WriteLine($"  Inserted: {fact}");
}
Console.WriteLine($"  Total: {db.FactCount()}, Active: {db.ActiveFactCount()}");

Console.WriteLine("\n--- Query All Facts ---");
using (var query = db.Query("SELECT * FROM facts"))
{
    foreach (var fact in query)
        Console.WriteLine($"  {fact}");
}

Console.WriteLine("\n--- Update Fact ---");
db.Update(0, new Fact(10, 0, 20, 0.99, evidence: 5, timestamp: 1700000003000000000, context: 3, version: 3, priority: 2, owner: 10));
Console.WriteLine($"  Updated row 0. Active count: {db.ActiveFactCount()}");

Console.WriteLine("\n--- Delete Fact ---");
db.Delete(1);
Console.WriteLine($"  Deleted row 1. Active count: {db.ActiveFactCount()}");

Console.WriteLine("\n--- Transaction Commit ---");
using (var txn = db.BeginTransaction())
{
    db.Insert(new Fact(4, 3, 5, 0.80));
    Console.WriteLine($"  Inserted in transaction. Count: {db.FactCount()}");
    txn.Commit();
    Console.WriteLine($"  Committed. Count: {db.FactCount()}");
}

Console.WriteLine("\n--- Transaction Rollback ---");
using (var txn2 = db.BeginTransaction())
{
    db.Insert(new Fact(5, 4, 6, 0.70));
    Console.WriteLine($"  Inserted in transaction. Count: {db.FactCount()}");
    txn2.Rollback();
    Console.WriteLine($"  Rolled back. Count: {db.FactCount()}");
}

Console.WriteLine("\n--- Save and Load ---");
string path = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "kcm_example.kcm");
try
{
    db.Save(path);
    Console.WriteLine($"  Saved to {path}");

    using var db2 = new KcmDatabase();
    db2.Load(path);
    Console.WriteLine($"  Loaded. Fact count: {db2.FactCount()}");

    KcmDatabase.Verify(path);
    Console.WriteLine("  Verified OK");
}
finally
{
    if (System.IO.File.Exists(path))
        System.IO.File.Delete(path);
}

Console.WriteLine("\n--- Error Handling ---");
try
{
    var bad = new Fact(1, 0, 2, 1.5);
    Console.WriteLine("  FAIL: Should have thrown");
}
catch (ArgumentException e)
{
    Console.WriteLine($"  Invalid confidence rejected: {e.Message}");
}

Console.WriteLine("\n--- KcmError Codes ---");
foreach (KcmError code in Enum.GetValues<KcmError>())
{
    var ex = new KcmException(code);
    Console.WriteLine($"  {code} ({(int)code}): {ex.Message}");
}

Console.WriteLine("\n=== All examples completed! ===");
