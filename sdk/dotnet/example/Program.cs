using System;
using Kcm;

Console.WriteLine("=== KCM .NET SDK Example ===\n");

// Create facts
var f1 = new Fact(1, 0, 2, 0.95);
var f2 = new Fact(2, 1, 3, 0.90);
var f3 = new Fact(3, 2, 4, 0.85);

Console.WriteLine($"Fact 1: {f1}");
Console.WriteLine($"Fact 2: {f2}");
Console.WriteLine($"Fact 3: {f3}");

// Test invalid confidence
try {
    var bad = new Fact(1, 0, 2, 1.5);
    Console.WriteLine("FAIL: Should have thrown");
} catch (ArgumentException e) {
    Console.WriteLine($"Invalid confidence rejected: {e.Message}");
}

Console.WriteLine("\nAll .NET SDK examples completed!");
