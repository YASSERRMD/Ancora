using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: multi-agent verifier -- run a primary agent and a verifier concurrently.
/// </summary>
public sealed class MultiAgentVerifierExampleTests
{
    [Fact]
    public async Task Both_Agents_Produce_Started_Events()
    {
        try
        {
            using var runtime = new Runtime();
            using var primaryAgent = new Agent(runtime);
            using var verifierAgent = new Agent(runtime);

            var primarySpec = new AgentSpec("local-model", "You are the primary agent.");
            var verifierSpec = new AgentSpec("local-model", "Verify the primary agent output.");

            var primaryTask = primaryAgent.Run(primarySpec).CollectAsync();
            var verifierTask = verifierAgent.Run(verifierSpec).CollectAsync();

            await Task.WhenAll(primaryTask, verifierTask);

            Assert.IsType<StartedEvent>(primaryTask.Result[0]);
            Assert.IsType<StartedEvent>(verifierTask.Result[0]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Both_Runs_Have_Distinct_RunIds()
    {
        try
        {
            using var runtime = new Runtime();
            using var primaryAgent = new Agent(runtime);
            using var verifierAgent = new Agent(runtime);

            var primaryHandle = primaryAgent.Run(new AgentSpec("local-model", "primary"));
            var verifierHandle = verifierAgent.Run(new AgentSpec("local-model", "verifier"));

            Assert.NotEqual(primaryHandle.RunId, verifierHandle.RunId);

            await primaryHandle.CollectAsync();
            await verifierHandle.CollectAsync();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Both_Runs_Complete()
    {
        try
        {
            using var runtime = new Runtime();
            using var a = new Agent(runtime);
            using var b = new Agent(runtime);

            var evA = await a.Run(new AgentSpec("local-model", "primary")).CollectAsync();
            var evB = await b.Run(new AgentSpec("local-model", "verifier")).CollectAsync();

            Assert.IsType<CompletedEvent>(evA.Last());
            Assert.IsType<CompletedEvent>(evB.Last());
        }
        catch (DllNotFoundException) { }
    }
}
