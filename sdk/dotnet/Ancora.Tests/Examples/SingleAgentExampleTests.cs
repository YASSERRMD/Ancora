using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Smoke tests for the single-agent example pattern.
/// Tests skip gracefully when the native library is not available.
/// </summary>
public sealed class SingleAgentExampleTests
{
    [Fact]
    public async Task SingleAgent_Runs_Without_Error()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "Be helpful.");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task SingleAgent_First_Event_Is_Started()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "You are a helpful assistant.");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.IsType<StartedEvent>(events[0]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task SingleAgent_Last_Event_Is_Completed()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "Respond briefly.");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.IsType<CompletedEvent>(events[events.Count - 1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task SingleAgent_RunId_Is_Not_Empty()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "Hello.");
            var handle = agent.Run(spec);
            Assert.NotEmpty(handle.RunId);
            await handle.CollectAsync();
        }
        catch (DllNotFoundException) { }
    }
}
