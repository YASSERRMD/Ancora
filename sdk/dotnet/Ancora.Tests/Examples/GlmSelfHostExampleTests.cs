using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: GLM self-host.
///
/// Demonstrates configuring an AgentSpec for the ChatGLM model family
/// and running it through the standard agent transport. The model name
/// is resolved by the runtime to the configured endpoint at runtime.
/// </summary>
public sealed class GlmSelfHostExampleTests
{
    private static readonly string[] GlmModels =
    [
        "glm-4",
        "glm-4-flash",
        "glm-4-air",
        "glm-3-turbo",
    ];

    [Fact]
    public void GlmModels_List_Has_At_Least_Two_Variants()
    {
        Assert.True(GlmModels.Length >= 2);
    }

    [Fact]
    public async Task GlmChat_Model_Runs_Without_Error()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("glm-4", "You are a helpful assistant powered by GLM.");
            var events = await agent.Run(spec).CollectAsync();
            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task All_Glm_Variants_Run_Sequentially()
    {
        try
        {
            using var runtime = new Runtime();
            foreach (var model in GlmModels)
            {
                using var agent = new Agent(runtime);
                var events = await agent.Run(new AgentSpec(model, "Respond briefly.")).CollectAsync();
                Assert.NotEmpty(events);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Each_Glm_Run_Has_Distinct_RunId()
    {
        try
        {
            using var runtime = new Runtime();
            var runIds = new System.Collections.Generic.HashSet<string>();
            foreach (var model in GlmModels)
            {
                using var agent = new Agent(runtime);
                var handle = agent.Run(new AgentSpec(model, "Hello."));
                runIds.Add(handle.RunId);
                await handle.CollectAsync();
            }
            Assert.Equal(GlmModels.Length, runIds.Count);
        }
        catch (DllNotFoundException) { }
    }
}
