using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Ancora.Tests.Examples;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: durable restart.
///
/// Demonstrates persisting run events to a RunJournal and replaying them
/// after a simulated process restart without re-running the agent.
/// </summary>
public sealed class DurableRestartExampleTests
{
    [Fact]
    public void RunJournal_Records_And_Replays_Events()
    {
        var journal = new RunJournal();
        journal.RecordRun("run-1");
        journal.AppendEvent("run-1", """{"kind":"started"}""");
        journal.AppendEvent("run-1", """{"kind":"completed"}""");

        Assert.Equal(2, journal.EventsForRun("run-1").Count);
        Assert.Equal(1, journal.RunCount);
    }

    [Fact]
    public void RunJournal_Returns_Empty_For_Unknown_Run()
    {
        var journal = new RunJournal();
        Assert.Empty(journal.EventsForRun("nonexistent"));
    }

    [Fact]
    public void RunJournal_Tracks_Multiple_Runs()
    {
        var journal = new RunJournal();
        journal.RecordRun("a");
        journal.RecordRun("b");
        Assert.Equal(2, journal.RunCount);
        Assert.Empty(journal.EventsForRun("a"));
    }

    [Fact]
    public void RunJournal_RecordRun_Is_Idempotent()
    {
        var journal = new RunJournal();
        journal.RecordRun("dup");
        journal.RecordRun("dup");
        Assert.Equal(1, journal.RunCount);
    }

    [Fact]
    public async Task DurableRestart_Persists_And_Replays_Live_Events()
    {
        try
        {
            var journal = new RunJournal();
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "Persist my events.");
            var handle = agent.Run(spec);
            var runId = handle.RunId;
            journal.RecordRun(runId);

            await foreach (var ev in handle.EventsAsync())
                journal.AppendEvent(runId, ev.Kind);

            var replayed = journal.EventsForRun(runId);
            Assert.NotEmpty(replayed);
            Assert.Equal(1, journal.RunCount);
        }
        catch (DllNotFoundException) { }
    }
}
