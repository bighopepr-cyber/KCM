using System;
using Xunit;
using Kcm;

namespace Kcm.Tests;

public class FactTests
{
    [Xunit.Fact]
    public void CreateFact_AllFields()
    {
        var f = new Fact(
            subject: 1,
            predicate: 2,
            obj: 3,
            confidence: 0.95,
            evidence: 4,
            timestamp: 1700000000000000000,
            context: 5,
            version: 10,
            priority: -1,
            owner: 42);

        Assert.Equal((uint)1, f.Subject);
        Assert.Equal((byte)2, f.Predicate);
        Assert.Equal((uint)3, f.Object);
        Assert.Equal(0.95, f.Confidence);
        Assert.Equal((byte)4, f.Evidence);
        Assert.Equal(1700000000000000000L, f.Timestamp);
        Assert.Equal((byte)5, f.Context);
        Assert.Equal(10, f.Version);
        Assert.Equal((sbyte)-1, f.Priority);
        Assert.Equal((ushort)42, f.Owner);
    }

    [Xunit.Fact]
    public void CreateFact_DefaultOptionalFields()
    {
        var f = new Fact(1, 0, 2, 0.95);

        Assert.Equal((uint)1, f.Subject);
        Assert.Equal((byte)0, f.Predicate);
        Assert.Equal((uint)2, f.Object);
        Assert.Equal(0.95, f.Confidence);
        Assert.Equal((byte)0, f.Evidence);
        Assert.Equal(0L, f.Timestamp);
        Assert.Equal((byte)0, f.Context);
        Assert.Equal(0, f.Version);
        Assert.Equal((sbyte)0, f.Priority);
        Assert.Equal((ushort)0, f.Owner);
    }

    [Xunit.Fact]
    public void CreateFact_ConfidenceBoundaries()
    {
        var fMin = new Fact(1, 0, 2, 0.0);
        Assert.Equal(0.0, fMin.Confidence);

        var fMax = new Fact(1, 0, 2, 1.0);
        Assert.Equal(1.0, fMax.Confidence);
    }

    [Xunit.Fact]
    public void CreateFact_InvalidConfidence_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Fact(1, 0, 2, 1.5));
        Assert.Throws<ArgumentException>(() => new Fact(1, 0, 2, -0.1));
        Assert.Throws<ArgumentException>(() => new Fact(1, 0, 2, double.NaN));
    }

    [Xunit.Fact]
    public void FactToString_ContainsAllFields()
    {
        var f = new Fact(1, 2, 3, 0.95, 4, 100, 5, 10, -1, 42);
        string s = f.ToString();

        Assert.Contains("Subject=1", s);
        Assert.Contains("Predicate=2", s);
        Assert.Contains("Object=3", s);
        Assert.Contains("0.95", s);
        Assert.Contains("Evidence=4", s);
        Assert.Contains("Timestamp=100", s);
        Assert.Contains("Context=5", s);
        Assert.Contains("Version=10", s);
        Assert.Contains("Priority=-1", s);
        Assert.Contains("Owner=42", s);
    }

    [Xunit.Fact]
    public void FactRoundTrip_NativeConversion()
    {
        var original = new Fact(10, 3, 20, 0.75, 7, 999, 2, 5, 3, 100);
        KCM_Fact native = original.ToNative();
        Fact restored = Fact.FromNative(native);

        Assert.Equal(original.Subject, restored.Subject);
        Assert.Equal(original.Predicate, restored.Predicate);
        Assert.Equal(original.Object, restored.Object);
        Assert.Equal(original.Confidence, restored.Confidence);
        Assert.Equal(original.Evidence, restored.Evidence);
        Assert.Equal(original.Timestamp, restored.Timestamp);
        Assert.Equal(original.Context, restored.Context);
        Assert.Equal(original.Version, restored.Version);
        Assert.Equal(original.Priority, restored.Priority);
        Assert.Equal(original.Owner, restored.Owner);
    }
}

public class KcmErrorTests
{
    [Theory]
    [InlineData(KcmError.Ok, 0)]
    [InlineData(KcmError.NotFound, 1)]
    [InlineData(KcmError.OutOfMemory, 2)]
    [InlineData(KcmError.InvalidArgument, 3)]
    [InlineData(KcmError.Io, 4)]
    [InlineData(KcmError.Corrupted, 5)]
    [InlineData(KcmError.Conflict, 6)]
    [InlineData(KcmError.TransactionAborted, 7)]
    public void KcmError_HasCorrectValues(KcmError error, int expected)
    {
        Assert.Equal(expected, (int)error);
    }

    [Xunit.Fact]
    public void KcmException_WithErrorCode()
    {
        var ex = new KcmException(KcmError.NotFound);
        Assert.Equal(KcmError.NotFound, ex.ErrorCode);
        Assert.NotNull(ex.Message);
        Assert.NotEmpty(ex.Message);
    }

    [Xunit.Fact]
    public void KcmException_WithMessage()
    {
        var ex = new KcmException("custom error");
        Assert.Equal("custom error", ex.Message);
        Assert.Equal(KcmError.InvalidArgument, ex.ErrorCode);
    }
}

public class DatabaseTests
{
    private KcmDatabase CreateDb() => new();

    [Xunit.Fact]
    public void CreateDatabase()
    {
        using var db = CreateDb();
        Assert.Equal(0UL, db.FactCount());
        Assert.Equal(0UL, db.ActiveFactCount());
    }

    [Xunit.Fact]
    public void Insert_IncreasesCount()
    {
        using var db = CreateDb();
        db.Insert(new Fact(1, 0, 2, 0.95));
        Assert.Equal(1UL, db.FactCount());
        Assert.Equal(1UL, db.ActiveFactCount());
    }

    [Xunit.Fact]
    public void Insert_MultipleFacts()
    {
        using var db = CreateDb();
        db.Insert(new Fact(1, 0, 2, 0.95));
        db.Insert(new Fact(2, 1, 3, 0.90));
        db.Insert(new Fact(3, 2, 4, 0.85));
        Assert.Equal(3UL, db.FactCount());
        Assert.Equal(3UL, db.ActiveFactCount());
    }

    [Xunit.Fact]
    public void Insert_AllFieldsPreserved()
    {
        using var db = CreateDb();
        var fact = new Fact(10, 3, 20, 0.75, 7, 999, 2, 5, 3, 100);
        db.Insert(fact);

        using var query = db.Query("SELECT * FROM facts");
        var results = new System.Collections.Generic.List<Fact>();
        foreach (var f in query)
            results.Add(f);

        Assert.Single(results);
        var r = results[0];
        Assert.Equal((uint)10, r.Subject);
        Assert.Equal((byte)3, r.Predicate);
        Assert.Equal((uint)20, r.Object);
        Assert.Equal(0.75, r.Confidence);
        Assert.Equal((byte)7, r.Evidence);
        Assert.Equal(999L, r.Timestamp);
        Assert.Equal((byte)2, r.Context);
        Assert.Equal(5, r.Version);
        Assert.Equal((sbyte)3, r.Priority);
        Assert.Equal((ushort)100, r.Owner);
    }

    [Xunit.Fact]
    public void Delete_DecreasesActiveCount()
    {
        using var db = CreateDb();
        db.Insert(new Fact(1, 0, 2, 0.95));
        db.Insert(new Fact(2, 1, 3, 0.90));
        db.Delete(0);
        Assert.Equal(2UL, db.FactCount());
        Assert.Equal(1UL, db.ActiveFactCount());
    }

    [Xunit.Fact]
    public void Update_ReplacesFact()
    {
        using var db = CreateDb();
        db.Insert(new Fact(1, 0, 2, 0.5));
        db.Update(0, new Fact(10, 0, 20, 0.99));

        using var query = db.Query("SELECT * FROM facts");
        var results = new System.Collections.Generic.List<Fact>();
        foreach (var f in query)
            results.Add(f);

        Assert.Single(results);
        Assert.Equal((uint)10, results[0].Subject);
        Assert.Equal(0.99, results[0].Confidence);
    }

    [Xunit.Fact]
    public void SaveAndLoad()
    {
        string path = System.IO.Path.GetTempFileName();
        try
        {
            using (var db = CreateDb())
            {
                db.Insert(new Fact(1, 0, 2, 0.95));
                db.Save(path);
            }

            using (var db2 = CreateDb())
            {
                db2.Load(path);
                Assert.Equal(1UL, db2.FactCount());
            }
        }
        finally
        {
            if (System.IO.File.Exists(path))
                System.IO.File.Delete(path);
        }
    }

    [Xunit.Fact]
    public void Verify_ValidFile()
    {
        string path = System.IO.Path.GetTempFileName();
        try
        {
            using (var db = CreateDb())
            {
                db.Insert(new Fact(1, 0, 2, 0.95));
                db.Save(path);
            }

            KcmDatabase.Verify(path);
        }
        finally
        {
            if (System.IO.File.Exists(path))
                System.IO.File.Delete(path);
        }
    }

    [Xunit.Fact]
    public void Query_EmptyDatabase()
    {
        using var db = CreateDb();
        using var query = db.Query("SELECT * FROM facts");
        var results = new System.Collections.Generic.List<Fact>();
        foreach (var f in query)
            results.Add(f);

        Assert.Empty(results);
    }

    [Xunit.Fact]
    public void Dispose_CanBeCalledMultipleTimes()
    {
        var db = CreateDb();
        db.Dispose();
        db.Dispose();
    }

    [Xunit.Fact]
    public void Dispose_ThrowsOnUseAfterDispose()
    {
        var db = CreateDb();
        db.Dispose();
        Assert.Throws<ObjectDisposedException>(() => db.Insert(new Fact(1, 0, 2, 0.95)));
    }
}

public class TransactionTests
{
    [Xunit.Fact]
    public void BeginAndCommit()
    {
        using var db = new KcmDatabase();
        using var txn = db.BeginTransaction();
        txn.Commit();
    }

    [Xunit.Fact]
    public void BeginAndRollback()
    {
        using var db = new KcmDatabase();
        using var txn = db.BeginTransaction();
        txn.Rollback();
    }

    [Xunit.Fact]
    public void Transaction_DisposeAfterCommit()
    {
        using var db = new KcmDatabase();
        using (var txn = db.BeginTransaction())
        {
            txn.Commit();
        }
    }

    [Xunit.Fact]
    public void Transaction_CommitThenDispose_ThrowsOnSecondCommit()
    {
        using var db = new KcmDatabase();
        var txn = db.BeginTransaction();
        txn.Commit();
        Assert.Throws<ObjectDisposedException>(() => txn.Commit());
    }
}

public class ErrorHandlingTests
{
    [Xunit.Fact]
    public void KcmException_ErrorMessage_NotNull()
    {
        foreach (KcmError code in Enum.GetValues<KcmError>())
        {
            var ex = new KcmException(code);
            Assert.NotNull(ex.Message);
            Assert.NotEmpty(ex.Message);
        }
    }

    [Xunit.Fact]
    public void Database_NotFound_Throws()
    {
        using var db = new KcmDatabase();
        string path = System.IO.Path.GetTempFileName();
        try
        {
            System.IO.File.Delete(path);
            Assert.Throws<KcmException>(() => db.Load(path));
        }
        finally
        {
            if (System.IO.File.Exists(path))
                System.IO.File.Delete(path);
        }
    }
}
