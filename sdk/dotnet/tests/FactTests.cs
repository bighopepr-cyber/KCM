using Xunit;
using Kcm;

namespace Kcm.Tests;

public class FactTests
{
    [Fact]
    public void CreateFact_ValidValues()
    {
        var f = new Fact(1, 0, 2, 0.95);
        Assert.Equal((uint)1, f.Subject);
        Assert.Equal((byte)0, f.Predicate);
        Assert.Equal((uint)2, f.Object);
        Assert.Equal(0.95, f.Confidence);
    }

    [Fact]
    public void CreateFact_InvalidConfidence_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Fact(1, 0, 2, 1.5));
        Assert.Throws<ArgumentException>(() => new Fact(1, 0, 2, -0.1));
    }

    [Fact]
    public void FactToString_ContainsInfo()
    {
        var f = new Fact(1, 0, 2, 0.95);
        Assert.Contains("Fact", f.ToString());
        Assert.Contains("Subject=1", f.ToString());
    }
}
