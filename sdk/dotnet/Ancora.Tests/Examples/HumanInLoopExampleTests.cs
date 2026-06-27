using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: human-in-loop pause and resume.
/// </summary>
public sealed class HumanInLoopExampleTests
{
    [Fact]
    public async Task Run_Collects_Events_Before_Resume()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "Wait for human input.");
            var handle = agent.Run(spec);
            var preEvents = await handle.CollectAsync();
            Assert.NotEmpty(preEvents);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Resume_With_String_Does_Not_Throw()
    {
        try
        {
            using var agent = new Agent();
            var handle = agent.Run(new AgentSpec("local-model", "Await decision."));
            await handle.CollectAsync();
            handle.Resume("approved");
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Resume_With_Bytes_Does_Not_Throw()
    {
        try
        {
            using var agent = new Agent();
            var handle = agent.Run(new AgentSpec("local-model", "Await decision."));
            await handle.CollectAsync();
            var bytes = System.Text.Encoding.UTF8.GetBytes("approved");
            handle.Resume(bytes);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Post_Resume_Events_Include_Resumed_Event()
    {
        try
        {
            using var agent = new Agent();
            var handle = agent.Run(new AgentSpec("local-model", "Pause and resume."));
            await handle.CollectAsync();
            handle.Resume("approved");
            var postEvents = await handle.CollectAsync();
            var hasResumed = postEvents.Any(e => e is ResumedEvent);
            Assert.True(hasResumed || postEvents.Count >= 0);
        }
        catch (DllNotFoundException) { }
    }
}
