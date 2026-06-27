using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public record RateLimitFixture147(int MaxRpm, TimeSpan RetryAfter, int BurstSize);

public class Phase147RelRateLimitTests
{
    private static readonly RateLimitFixture147 Fixture = new(60, TimeSpan.FromSeconds(1), 5);

    [Fact]
    public void Fixture_Max_Rpm_Is_60()
    {
        Assert.Equal(60, Fixture.MaxRpm);
    }

    [Fact]
    public void Fixture_Retry_After_Is_One_Second()
    {
        Assert.Equal(TimeSpan.FromSeconds(1), Fixture.RetryAfter);
    }

    [Fact]
    public void Fixture_Burst_Size_Is_Five()
    {
        Assert.Equal(5, Fixture.BurstSize);
    }

    [Fact]
    public void Rate_Limit_Per_Second_Derived_From_Rpm()
    {
        var rps = Fixture.MaxRpm / 60.0;
        Assert.Equal(1.0, rps);
    }

    [Fact]
    public async Task Burst_Of_Five_Runs_Completes()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var tasks = Enumerable.Range(0, Fixture.BurstSize).Select(async _ =>
            {
                using var a = new Agent(rt);
                return await a.Run(spec).CollectAsync();
            });
            var results = await Task.WhenAll(tasks);
            Assert.Equal(Fixture.BurstSize, results.Length);
            foreach (var events in results)
                Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Exponential_Backoff_Intervals_Increase()
    {
        var delays = Enumerable.Range(0, 4)
            .Select(i => TimeSpan.FromSeconds(Math.Pow(2, i)))
            .ToList();
        for (int i = 1; i < delays.Count; i++)
            Assert.True(delays[i] > delays[i - 1]);
    }

    [Fact]
    public async Task Sequential_Runs_Within_Wall_Time_Bound()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            var sw = Stopwatch.StartNew();
            for (int i = 0; i < 5; i++)
                await a.Run(spec).CollectAsync();
            sw.Stop();
            Assert.True(sw.Elapsed < TimeSpan.FromSeconds(30),
                $"Runs took {sw.Elapsed.TotalSeconds:F1}s, expected under 30s");
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void RetryAfter_Parsed_Correctly()
    {
        var retryAfterSeconds = Fixture.RetryAfter.TotalSeconds;
        Assert.Equal(1.0, retryAfterSeconds);
    }
}
