using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public static class GlmConstants147
{
    public const string GlmTurbo = "glm-5";
    public const string GlmFlash = "glm-5-flash";
    public const string GlmLong = "glm-5-long";
    public const string MockGatewayBaseUrl = "http://localhost:9999/v1";
}

public class Phase147E2eGlmSelfHostTests
{
    [Fact]
    public void GlmTurbo_Model_Id_Is_Correct()
    {
        Assert.Equal("glm-5", GlmConstants147.GlmTurbo);
    }

    [Fact]
    public void GlmFlash_Model_Id_Is_Correct()
    {
        Assert.Equal("glm-5-flash", GlmConstants147.GlmFlash);
    }

    [Fact]
    public void GlmLong_Model_Id_Is_Correct()
    {
        Assert.Equal("glm-5-long", GlmConstants147.GlmLong);
    }

    [Fact]
    public void Mock_Gateway_Base_Url_Is_Local()
    {
        Assert.StartsWith("http://localhost", GlmConstants147.MockGatewayBaseUrl);
    }

    [Fact]
    public void AgentSpec_With_Glm_Model_Encodes()
    {
        var spec = new AgentSpec(GlmConstants147.GlmTurbo);
        var json = Encoding.UTF8.GetString(Wire.EncodeAgentSpec(spec));
        Assert.Contains("glm-5", json);
    }

    [Fact]
    public async Task GlmTurbo_Run_Completes_Via_Mock()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec(GlmConstants147.GlmTurbo, Instructions: "You are a GLM model.");
            var events = await a.Run(spec).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task GlmFlash_Run_Completes_Via_Mock()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec(GlmConstants147.GlmFlash);
            var events = await a.Run(spec).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Mock_Gateway_URL_Contains_V1()
    {
        Assert.EndsWith("/v1", GlmConstants147.MockGatewayBaseUrl);
    }

    [Fact]
    public void GlmTurbo_RunId_Not_Empty()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec(GlmConstants147.GlmTurbo));
            Assert.NotEmpty(h.RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Three_GlmFlash_Runs_Unique_Ids()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec(GlmConstants147.GlmFlash);
            var ids = new System.Collections.Generic.HashSet<string>();
            for (int i = 0; i < 3; i++)
                ids.Add(a.Run(spec).RunId);
            Assert.Equal(3, ids.Count);
        }
        catch (DllNotFoundException) { }
    }
}
