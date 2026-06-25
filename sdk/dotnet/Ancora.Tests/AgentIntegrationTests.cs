using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Integration tests that drive a run through the Agent and RunHandle APIs.
/// Tests skip gracefully when the native library is not available.
/// CI builds the native library before running these tests.
/// </summary>
public class AgentIntegrationTests
{
    [Fact]
    public async Task Agent_Run_Produces_At_Least_One_Event()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("test-model", "Be brief");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.True(events.Count > 0, "Expected at least one event from run");
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public async Task Agent_Run_Emits_Started_Then_Completed()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("test-model", "You are helpful");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.True(events.Count >= 2, "Expected started + completed events");
            Assert.IsType<StartedEvent>(events[0]);
            Assert.IsType<CompletedEvent>(events[events.Count - 1]);
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public async Task Agent_Run_StartedEvent_Contains_RunId()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("test-model", "Hello");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            var started = events.OfType<StartedEvent>().FirstOrDefault();
            Assert.NotNull(started);
            Assert.Equal(handle.RunId, started.RunId);
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public async Task Agent_Resume_Appends_Resumed_And_Completed_Events()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("test-model", "Await human input");
            var handle = agent.Run(spec);
            // consume the initial events
            await handle.CollectAsync();
            // inject a decision
            var afterResume = await handle.ResumeAndCollectAsync("approved");
            Assert.True(afterResume.Count >= 2, "Expected resumed + completed after resume");
            Assert.IsType<ResumedEvent>(afterResume[0]);
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public void Agent_Shared_Runtime_Does_Not_Dispose_Runtime()
    {
        try
        {
            using var rt = new Runtime();
            var agent1 = new Agent(rt);
            agent1.Dispose();
            // runtime must still be usable after agent1 is disposed
            var agent2 = new Agent(rt);
            agent2.Dispose();
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public async Task RunHandle_GetCost_Returns_Json()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("test-model", "Cost test");
            var handle = agent.Run(spec);
            await handle.CollectAsync();
            var cost = handle.GetCost();
            Assert.NotNull(cost);
            Assert.Contains("run_id", cost);
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }
}
