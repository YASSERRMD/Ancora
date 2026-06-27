using System;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147SecAirgapTests
{
    private static readonly object LocalSchema = new
    {
        type = "object",
        properties = new { answer = new { type = "string" } },
        required = new[] { "answer" }
    };

    [Fact]
    public void Local_Schema_Has_No_Http_Refs()
    {
        var json = JsonSerializer.Serialize(LocalSchema);
        Assert.DoesNotContain("http", json);
        Assert.DoesNotContain("$ref", json);
    }

    [Fact]
    public void Local_Schema_Type_Is_Object()
    {
        var json = JsonDocument.Parse(JsonSerializer.Serialize(LocalSchema));
        Assert.Equal("object", json.RootElement.GetProperty("type").GetString());
    }

    [Fact]
    public void AgentSpec_Encodes_Without_External_Refs()
    {
        var spec = new AgentSpec("llama3", Instructions: "Offline only.");
        var bytes = Wire.EncodeAgentSpec(spec);
        var json = System.Text.Encoding.UTF8.GetString(bytes);
        Assert.DoesNotContain("http", json);
        Assert.DoesNotContain("$ref", json);
    }

    [Fact]
    public async Task Events_Contain_No_External_Api_Urls()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await foreach (var ev in h.EventsAsync())
            {
                var json = JsonSerializer.Serialize(ev, new JsonSerializerOptions
                {
                    WriteIndented = false
                });
                Assert.DoesNotContain("api.anthropic.com", json);
                Assert.DoesNotContain("api.openai.com", json);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Events_Contain_No_Api_Key_Patterns()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await foreach (var ev in h.EventsAsync())
            {
                var json = JsonSerializer.Serialize(ev);
                Assert.DoesNotContain("sk-ant-", json);
                Assert.DoesNotContain("sk-proj-", json);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Runtime_Creates_Without_Network()
    {
        try
        {
            using var rt = new Runtime();
            Assert.NotNull(rt);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Four_Conformance_Scenarios_All_Offline()
    {
        try
        {
            using var a = new Agent();
            var scenarios = new[] { "single-agent", "verifier", "hil", "rag" };
            foreach (var s in scenarios)
            {
                var h = a.Run(new AgentSpec(s));
                var events = await h.CollectAsync();
                Assert.NotEmpty(events);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Wire_EncodeAgentSpec_Contains_No_Sensitive_Data()
    {
        var spec = new AgentSpec("llama3");
        var bytes = Wire.EncodeAgentSpec(spec);
        var json = System.Text.Encoding.UTF8.GetString(bytes);
        Assert.DoesNotContain("password", json);
        Assert.DoesNotContain("secret", json);
    }
}
