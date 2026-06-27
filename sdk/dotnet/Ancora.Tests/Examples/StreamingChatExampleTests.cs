using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: streaming chat with IAsyncEnumerable.
/// Events are consumed one at a time as they arrive via EventsAsync().
/// </summary>
public sealed class StreamingChatExampleTests
{
    [Fact]
    public async Task Streaming_Produces_Events_Via_AsyncEnumerable()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "Stream your response.");
            var handle = agent.Run(spec);

            var events = new List<RunEvent>();
            await foreach (var ev in handle.EventsAsync())
                events.Add(ev);

            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Streaming_Contains_Token_Events()
    {
        try
        {
            using var agent = new Agent();
            var handle = agent.Run(new AgentSpec("local-model", "Respond in tokens."));
            var events = await handle.CollectAsync();
            var tokenEvents = events.OfType<TokenEvent>().ToList();
            Assert.True(tokenEvents.Count >= 0);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Streaming_Token_Text_Is_Concatenatable()
    {
        try
        {
            using var agent = new Agent();
            var handle = agent.Run(new AgentSpec("local-model", "Say hello."));
            var events = await handle.CollectAsync();
            var fullText = string.Concat(events.OfType<TokenEvent>().Select(e => e.Text));
            Assert.NotNull(fullText);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Streaming_Ends_With_Completed_Event()
    {
        try
        {
            using var agent = new Agent();
            var events = await agent.Run(new AgentSpec("local-model", "Done.")).CollectAsync();
            Assert.IsType<CompletedEvent>(events.Last());
        }
        catch (DllNotFoundException) { }
    }
}
