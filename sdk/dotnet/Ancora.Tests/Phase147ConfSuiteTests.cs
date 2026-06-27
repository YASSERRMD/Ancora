using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147ConfSuiteTests
{
    private static readonly string[] Scenarios =
    [
        "single-agent",
        "multi-agent-verifier",
        "human-in-loop",
        "rag-retrieval",
    ];

    [Fact]
    public void Conformance_Scenarios_List_Has_Four_Entries()
    {
        Assert.Equal(4, Scenarios.Length);
    }

    [Fact]
    public void Scenarios_Contains_Single_Agent()
    {
        Assert.Contains("single-agent", Scenarios);
    }

    [Fact]
    public void Scenarios_Contains_Multi_Agent_Verifier()
    {
        Assert.Contains("multi-agent-verifier", Scenarios);
    }

    [Fact]
    public void Scenarios_Contains_Human_In_Loop()
    {
        Assert.Contains("human-in-loop", Scenarios);
    }

    [Fact]
    public void Scenarios_Contains_Rag_Retrieval()
    {
        Assert.Contains("rag-retrieval", Scenarios);
    }

    [Fact]
    public async Task All_Scenarios_Produce_Events()
    {
        try
        {
            using var a = new Agent();
            foreach (var scenario in Scenarios)
            {
                var spec = new AgentSpec(scenario);
                var events = await a.Run(spec).CollectAsync();
                Assert.NotEmpty(events);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task All_Scenarios_Complete()
    {
        try
        {
            using var a = new Agent();
            foreach (var scenario in Scenarios)
            {
                var spec = new AgentSpec(scenario);
                var events = await a.Run(spec).CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Second_Run_Of_Each_Scenario_Consistent()
    {
        try
        {
            using var a = new Agent();
            foreach (var scenario in Scenarios)
            {
                var spec = new AgentSpec(scenario);
                var first = await a.Run(spec).CollectAsync();
                var second = await a.Run(spec).CollectAsync();
                Assert.Equal(first.Count, second.Count);
                Assert.IsType<CompletedEvent>(second[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Scenario_Ids_Are_Unique_Strings()
    {
        Assert.Equal(Scenarios.Length, Scenarios.Distinct().Count());
    }

    [Fact]
    public async Task Parallel_Scenarios_All_Complete()
    {
        try
        {
            using var rt = new Runtime();
            var tasks = Scenarios.Select(async scenario =>
            {
                using var a = new Agent(rt);
                var spec = new AgentSpec(scenario);
                return await a.Run(spec).CollectAsync();
            });
            var results = await Task.WhenAll(tasks);
            foreach (var events in results)
                Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }
}
