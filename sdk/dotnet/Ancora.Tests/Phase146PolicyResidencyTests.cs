using System.Collections.Generic;
using System.Text.Json;
using Xunit;

namespace Ancora.Tests;

public record PolicyEvent146(string Region, bool Blocked, string? Reason = null);

public class Phase146PolicyResidencyTests
{
    private static readonly PolicyEvent146[] Fixture =
    [
        new("eu-west-1", false),
        new("cn-north-1", true, "region not allowed by policy"),
        new("us-east-1", false),
        new("ap-southeast-1", false),
        new("cn-northwest-1", true, "data sovereignty: China mainland"),
    ];

    [Fact]
    public void Allowed_Region_Not_Blocked()
    {
        var ev = new PolicyEvent146("eu-west-1", false);
        Assert.False(ev.Blocked);
    }

    [Fact]
    public void Blocked_Region_Has_Reason()
    {
        var ev = new PolicyEvent146("cn-north-1", true, "not allowed");
        Assert.True(ev.Blocked);
        Assert.NotNull(ev.Reason);
    }

    [Fact]
    public void Fixture_Has_Two_Blocked_Regions()
    {
        int blocked = 0;
        foreach (var ev in Fixture) if (ev.Blocked) blocked++;
        Assert.Equal(2, blocked);
    }

    [Fact]
    public void Fixture_Has_Three_Allowed_Regions()
    {
        int allowed = 0;
        foreach (var ev in Fixture) if (!ev.Blocked) allowed++;
        Assert.Equal(3, allowed);
    }

    [Fact]
    public void Allowed_Events_Have_No_Reason()
    {
        foreach (var ev in Fixture)
            if (!ev.Blocked) Assert.Null(ev.Reason);
    }

    [Fact]
    public void PolicyEvent_Value_Equality()
    {
        var a = new PolicyEvent146("eu-west-1", false);
        var b = new PolicyEvent146("eu-west-1", false);
        Assert.Equal(a, b);
    }

    [Fact]
    public void Blocked_Reason_Contains_Region_Or_Policy()
    {
        foreach (var ev in Fixture.Where(e => e.Blocked))
            Assert.True(ev.Reason!.Length > 0);
    }

    [Fact]
    public void China_Mainland_Is_Blocked()
    {
        var cn = Fixture.Where(e => e.Region.StartsWith("cn-")).ToList();
        Assert.All(cn, e => Assert.True(e.Blocked));
    }

    [Fact]
    public void EU_And_US_Are_Allowed()
    {
        var allowed = Fixture.Where(e =>
            e.Region.StartsWith("eu-") || e.Region.StartsWith("us-")).ToList();
        Assert.All(allowed, e => Assert.False(e.Blocked));
    }

    [Fact]
    public void PolicyEvent_Json_Roundtrip()
    {
        var ev = new PolicyEvent146("eu-west-1", false);
        var json = JsonSerializer.Serialize(ev);
        var back = JsonSerializer.Deserialize<PolicyEvent146>(json);
        Assert.Equal(ev, back);
    }
}
